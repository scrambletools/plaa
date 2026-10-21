//! The model side of plaa's assistant panel. A [`Chat`] talks to one
//! language model on a thread of its own, with a tokio runtime, as iced has
//! its own executor. plaa sends it what the user says and the results of
//! the tools the model calls; it sends back the reply as it streams, the
//! tool calls to run, and the end of each turn.
//!
//! rig-core is the only model library here, and nothing outside this crate
//! knows it, so a new rig version touches only this file.

mod models;

use std::collections::HashMap;
use std::sync::{Arc, mpsc};

use base64::Engine;
use futures::StreamExt;
use rig_core::DynModel;
use rig_core::completion::message::{
    AssistantContent, DocumentSourceKind, Image, ImageMediaType, Message, ToolResultContent,
    UserContent,
};
use rig_core::completion::{CompletionRequest, ToolDefinition};
use rig_core::error::ProviderError;
use rig_core::operation::Completion;
use rig_core::streaming::{Item, StreamEvent};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub use models::{
    ModelInfo, billing_page, find_server, friendly_name, key_page, list, server_page,
};

/// Who serves a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Provider {
    Anthropic,
    OpenAi,
    Gemini,
    /// A local Ollama, at its address.
    Ollama,
    /// Any server that speaks OpenAI's chat API, at its address.
    OpenAiCompatible,
}

impl Provider {
    pub const ALL: [Provider; 5] = [
        Provider::Anthropic,
        Provider::OpenAi,
        Provider::Gemini,
        Provider::Ollama,
        Provider::OpenAiCompatible,
    ];

    /// Whether the provider needs an API key.
    pub fn needs_key(self) -> bool {
        matches!(
            self,
            Provider::Anthropic | Provider::OpenAi | Provider::Gemini
        )
    }

    /// Whether the provider is reached at an address the user gives.
    /// Whether plaa tells the server how much context to keep: Ollama
    /// takes it with each request, while other servers set their own.
    pub fn sets_context(self) -> bool {
        self == Provider::Ollama
    }

    pub fn needs_address(self) -> bool {
        matches!(self, Provider::Ollama | Provider::OpenAiCompatible)
    }

    /// The address used when none is given.
    pub fn default_address(self) -> Option<&'static str> {
        match self {
            Provider::Ollama => Some("http://localhost:11434"),
            Provider::OpenAiCompatible => Some("http://localhost:8080/v1"),
            _ => None,
        }
    }
}

/// A model to chat with: its provider, its name and, for a server the user
/// runs, its address. The key, when one is needed, is kept apart from this,
/// in the system keychain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ModelChoice {
    pub provider: Provider,
    pub model: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// How much a local model reads at once, in tokens, where its server
    /// takes it from plaa: [`DEFAULT_CONTEXT`] when not given.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<u32>,
    /// Whether the model sees pictures: `Some(false)` gets the render
    /// tools' text without their pictures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vision: Option<bool>,
}

/// The context a local model gets unless Settings gives another, in
/// tokens. Ollama's own default, 4096, is too small for plaa's tools, whose
/// definitions alone take about 9000; a larger one costs memory, as the
/// server keeps room for all of it.
pub const DEFAULT_CONTEXT: u32 = 32_768;

/// How long a trial waits before trying again after a rate limit.
const RATE_WAIT: std::time::Duration = std::time::Duration::from_secs(10);

/// The longest reply asked of an Anthropic model rig does not know, in
/// tokens: within every current Claude model's limit.
const ANTHROPIC_REPLY: u64 = 8_192;

/// The context sizes Settings offers.
pub const CONTEXT_SIZES: [u32; 6] = [8_192, 16_384, 32_768, 65_536, 131_072, 262_144];

impl ModelChoice {
    /// The longest reply to ask for, where rig sets none: Anthropic needs
    /// one in every request, and rig knows the limits of the models it was
    /// released with only.
    fn max_tokens(&self) -> Option<u64> {
        let known = rig_core::providers::anthropic::ANTHROPIC.default_max_tokens(self.model.trim());
        (self.provider == Provider::Anthropic && known.is_none()).then_some(ANTHROPIC_REPLY)
    }

    /// The model's context in tokens, where plaa knows it: the one Settings
    /// gives, or the one plaa gives a local server's model. A cloud
    /// model's is not known, as it differs from model to model, and is
    /// often far more than a chat needs.
    fn context(&self) -> Option<u64> {
        let default = match self.provider {
            Provider::Ollama | Provider::OpenAiCompatible => Some(DEFAULT_CONTEXT),
            _ => None,
        };
        self.context.or(default).map(u64::from)
    }

    /// What the provider needs in each request besides the messages.
    fn request_params(&self) -> Option<Value> {
        match self.provider {
            Provider::Ollama => Some(serde_json::json!({
                "num_ctx": self.context.unwrap_or(DEFAULT_CONTEXT),
            })),
            _ => None,
        }
    }

    /// The rig model for this choice, with `key` where the provider needs one.
    fn connect(&self, key: Option<&str>) -> Result<DynModel<Completion>, String> {
        use rig_core::providers::{anthropic, gemini, ollama, openai};
        let key = key.unwrap_or_default().to_owned();
        if self.provider.needs_key() && key.trim().is_empty() {
            return Err("This model needs an API key.".to_owned());
        }
        let address = self
            .address
            .clone()
            .filter(|address| !address.trim().is_empty())
            .or_else(|| self.provider.default_address().map(str::to_owned));
        let model = self.model.trim();
        if model.is_empty() {
            return Err("Give the model's name.".to_owned());
        }
        Ok(match self.provider {
            // A cloud provider takes an address only to be reached
            // another way, as through a proxy or the recorded tests' server.
            Provider::Anthropic => {
                let mut config = anthropic::AnthropicConfig::new(key);
                if let Some(address) = address {
                    config.base_url = address;
                }
                // Each request starts with the same tools and instructions
                // and the conversation so far, so Anthropic reads most of it
                // from its cache, which it does only when asked. The other
                // providers cache a prefix they have seen on their own.
                let mut model = config.client().completion(model);
                model.wire = model.wire.with_automatic_caching();
                model.erase()
            }
            Provider::OpenAi => {
                let mut config = openai::OpenAIConfig::new(key);
                if let Some(address) = address {
                    config.base_url = address;
                }
                config.client().completion(model).erase()
            }
            Provider::Gemini => {
                let mut config = gemini::GeminiConfig::new(key);
                if let Some(address) = address {
                    config.base_url = address;
                }
                config.client().completion(model).erase()
            }
            Provider::Ollama => {
                let mut config = ollama::OllamaConfig::new();
                if let Some(address) = address {
                    config.base_url = address;
                }
                config.client().completion(model).erase()
            }
            // Chat Completions, which compatible servers speak, rather than
            // the Responses API rig picks for OpenAI's own.
            Provider::OpenAiCompatible => {
                let mut config = openai::OpenAIConfig::new(key);
                if let Some(address) = address {
                    config.base_url = address;
                }
                config.client().chat(model).erase()
            }
        })
    }
}

/// API keys, kept in the system's keychain: the Keychain on macOS, the
/// Credential Manager on Windows, and the Secret Service on Linux. Each
/// model's key is under its id, in `service`.
pub mod keys {
    fn entry(service: &str, id: &str) -> Result<keyring::Entry, String> {
        keyring::Entry::new(service, id).map_err(|error| error.to_string())
    }

    pub fn set(service: &str, id: &str, key: &str) -> Result<(), String> {
        entry(service, id)?
            .set_password(key)
            .map_err(|error| error.to_string())
    }

    /// The key kept for `id`, or `None` when there is none.
    pub fn get(service: &str, id: &str) -> Result<Option<String>, String> {
        match entry(service, id)?.get_password() {
            Ok(key) => Ok(Some(key)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn delete(service: &str, id: &str) -> Result<(), String> {
        match entry(service, id)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}

/// A model name to show as an example for each provider.
pub fn example_model(provider: Provider) -> &'static str {
    match provider {
        Provider::Anthropic => rig_core::providers::anthropic::CLAUDE_SONNET_5_5,
        Provider::OpenAi => rig_core::providers::openai::GPT_5_6,
        Provider::Gemini => "gemini-3.8-flash",
        Provider::Ollama => "qwen3.8",
        Provider::OpenAiCompatible => "local-model",
    }
}

/// A tool the model may call: plaa's tool registry, as JSON schemas.
#[derive(Debug, Clone)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub schema: Value,
}

/// What a tool gave back.
#[derive(Debug, Clone)]
pub enum Content {
    Text(String),
    Png(Vec<u8>),
}

/// A call the model asks plaa to run.
#[derive(Debug, Clone)]
pub struct ToolCall {
    /// Tells this call's result from the others in the same turn.
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

/// A tool call's result, for the model.
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub id: String,
    pub content: Vec<Content>,
    pub failed: bool,
}

/// What a chat tells plaa.
#[derive(Debug, Clone)]
pub enum Event {
    /// More of the model's reply.
    Text(String),
    /// More of what the model thinks before it replies, from models that
    /// reason.
    Reasoning(String),
    /// Tools to run; the chat waits for their results.
    ToolCalls(Vec<ToolCall>),
    /// The model finished its turn.
    Done,
    /// The turn failed; the chat can go on.
    Failed(Problem),
    /// The conversation is near the end of the model's context, so the
    /// model is summarizing its earlier part before it answers.
    Compacting,
    /// The earlier part of the conversation was replaced by its summary.
    Compacted,
    /// The conversation did not fit the model's context, so the long tool
    /// results of its earlier part were left out.
    Shortened,
    /// The user stopped the turn.
    Stopped,
}

/// Why a model did not answer, sorted so plaa can say it plainly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Problem {
    /// The conversation and the tools no longer fit the model's context.
    ContextTooSmall,
    /// The provider turned the API key down.
    Key,
    /// The provider asks to slow down, with what it said: which limit,
    /// and often how long to wait.
    RateLimited(String),
    /// The account has no credit left, or never had any: a new OpenAI or
    /// Anthropic account needs some bought before its key works.
    NoCredit,
    /// The provider has no model by that name.
    NoSuchModel,
    /// The provider's servers are busy or failing.
    Unavailable,
    /// The server could not be reached.
    Unreachable,
    /// The model declined to answer.
    Refused,
    /// Anything else, as the provider or plaa put it.
    Other(String),
}

impl Problem {
    fn of(error: &ProviderError) -> Problem {
        let response = match error {
            ProviderError::Http(_) => return Problem::Unreachable,
            ProviderError::InvalidAuthentication(_) => return Problem::Key,
            ProviderError::ProviderResponse(response) => response,
            other => return Problem::Other(other.to_string()),
        };
        let body = response.body.to_lowercase();
        if body.contains("context") && (body.contains("exceed") || body.contains("length"))
            || body.contains("prompt is too long")
            || body.contains("too many tokens")
        {
            return Problem::ContextTooSmall;
        }
        if response.refusal {
            return Problem::Refused;
        }
        // OpenAI's 429 insufficient_quota, Anthropic's 400 about the
        // credit balance, and anyone's 402.
        if body.contains("insufficient_quota")
            || body.contains("credit balance")
            || body.contains("billing")
            || response.status.is_some_and(|status| status.as_u16() == 402)
        {
            return Problem::NoCredit;
        }
        match response.status.map(|status| status.as_u16()) {
            Some(401 | 403) => Problem::Key,
            Some(429) => Problem::RateLimited(provider_message(&response.body)),
            Some(404) => Problem::NoSuchModel,
            Some(500..=599) => Problem::Unavailable,
            _ => Problem::Other(provider_message(&response.body)),
        }
    }
}

/// The message in a provider's error body, which is JSON for most:
/// `{"error":{"message":…}}`, `{"error":"…"}` or `{"message":…}`. The
/// body itself when it is not.
fn provider_message(body: &str) -> String {
    let Ok(value) = serde_json::from_str::<Value>(body) else {
        return body.trim().to_owned();
    };
    let message = [
        &value["error"]["message"],
        &value["error"],
        &value["message"],
        &value["detail"],
    ]
    .into_iter()
    .find_map(|field| field.as_str());
    match message {
        // Some servers put a JSON body in the message again.
        Some(message) if message.trim_start().starts_with('{') => provider_message(message),
        Some(message) => message.trim().to_owned(),
        None => body.trim().to_owned(),
    }
}

enum Command {
    Say(String),
    Results(Vec<ToolResult>),
    Model(ModelChoice, Option<String>),
    Stop,
}

/// A conversation with a model, on its own thread. Dropping it ends the
/// conversation.
pub struct Chat {
    commands: tokio::sync::mpsc::UnboundedSender<Command>,
    /// Wakes a turn under way to stop it.
    stop: Arc<tokio::sync::Notify>,
}

impl Chat {
    /// Starts a conversation with `choice`, which follows `instructions`
    /// and may call `tools`; `on_event` hears everything the chat says,
    /// on the chat's thread.
    pub fn start(
        choice: ModelChoice,
        key: Option<String>,
        instructions: String,
        tools: Vec<ToolSpec>,
        on_event: impl Fn(Event) + Send + 'static,
    ) -> Result<Chat, String> {
        let (commands, receiver) = tokio::sync::mpsc::unbounded_channel();
        let (started, ready) = mpsc::channel();
        let stop = Arc::new(tokio::sync::Notify::new());
        let stopping = stop.clone();
        std::thread::Builder::new()
            .name("plaa-assist".into())
            .spawn(move || {
                let runtime = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(runtime) => runtime,
                    Err(error) => {
                        let _ = started.send(Err(error.to_string()));
                        return;
                    }
                };
                let model = match choice.connect(key.as_deref()) {
                    Ok(model) => model,
                    Err(error) => {
                        let _ = started.send(Err(error));
                        return;
                    }
                };
                let _ = started.send(Ok(()));
                let conversation = Conversation {
                    model,
                    params: choice.request_params(),
                    max_tokens: choice.max_tokens(),
                    vision: choice.vision != Some(false),
                    pictures_in_results: choice.provider != Provider::Ollama,
                    context: choice.context(),
                    last_input: 0,
                    instructions,
                    tools: tools
                        .into_iter()
                        .map(|tool| ToolDefinition {
                            name: tool.name,
                            description: tool.description,
                            parameters: tool.schema,
                        })
                        .collect(),
                    history: Vec::new(),
                    waiting: HashMap::new(),
                };
                runtime.block_on(conversation.run(receiver, stopping, on_event));
            })
            .map_err(|error| error.to_string())?;
        ready
            .recv()
            .map_err(|_| "The assistant stopped.".to_owned())??;
        Ok(Chat { commands, stop })
    }

    /// Goes on with `choice` from the next turn, keeping the conversation.
    pub fn set_model(&self, choice: ModelChoice, key: Option<String>) {
        let _ = self.commands.send(Command::Model(choice, key));
    }

    /// Stops the turn under way, and any tool calls it is waiting for.
    pub fn stop(&self) {
        self.stop.notify_waiters();
        let _ = self.commands.send(Command::Stop);
    }

    /// The user says `text`.
    pub fn say(&self, text: impl Into<String>) {
        let _ = self.commands.send(Command::Say(text.into()));
    }

    /// The results of the tool calls of the last [`Event::ToolCalls`].
    pub fn results(&self, results: Vec<ToolResult>) {
        let _ = self.commands.send(Command::Results(results));
    }
}

/// Sends `choice` one short message, to check its name, address and key.
/// Blocks: call it off the interface thread.
pub fn test(choice: &ModelChoice, key: Option<&str>) -> Result<(), Problem> {
    let model = choice.connect(key).map_err(Problem::Other)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| Problem::Other(error.to_string()))?;
    // No small cap on the reply: a reasoning model may think before its
    // OK, and one cut off before it says anything reads as a failure.
    let mut request = CompletionRequest::new(Message::user("Reply with OK."));
    if let Some(tokens) = choice.max_tokens() {
        request = request.max_tokens(tokens);
    }
    // With the chat's own parameters, so Ollama loads the model at the
    // context the chat will use, and does not load it again for it.
    if let Some(params) = choice.request_params() {
        request = request.additional_params(params);
    }
    runtime.block_on(async move {
        let mut tries = 0;
        loop {
            match model.call(request.clone()).await {
                Ok(_) => return Ok(()),
                Err(error) => match Problem::of(&error) {
                    // A new account's first requests often meet a limit
                    // that lifts within seconds: wait once and try again.
                    Problem::RateLimited(_) if tries == 0 => {
                        tries += 1;
                        tokio::time::sleep(RATE_WAIT).await;
                    }
                    problem => return Err(problem),
                },
            }
        }
    })
}

struct Conversation {
    model: DynModel<Completion>,
    params: Option<Value>,
    max_tokens: Option<u64>,
    /// Whether the model gets the tools' pictures.
    vision: bool,
    /// Whether pictures can go in a tool result, which Ollama refuses; there
    /// they follow the results in the same message.
    pictures_in_results: bool,
    /// The model's context, where known, and how much of it the last
    /// request took.
    context: Option<u64>,
    last_input: u64,
    instructions: String,
    tools: Vec<ToolDefinition>,
    history: Vec<Message>,
    /// The tool calls the last turn asked for, by the id plaa knows them by.
    waiting: HashMap<String, rig_core::completion::message::ToolCall>,
}

impl Conversation {
    async fn run(
        mut self,
        mut commands: tokio::sync::mpsc::UnboundedReceiver<Command>,
        stop: Arc<tokio::sync::Notify>,
        on_event: impl Fn(Event),
    ) {
        while let Some(command) = commands.recv().await {
            match command {
                Command::Say(text) => {
                    self.answer_waiting();
                    if self.nearly_full() {
                        let stopped = tokio::select! {
                            () = self.compact(&on_event) => false,
                            () = stop.notified() => true,
                        };
                        if stopped {
                            self.history.push(Message::user(text));
                            on_event(Event::Stopped);
                            continue;
                        }
                    }
                    self.history.push(Message::user(text));
                }
                Command::Stop => {
                    self.answer_waiting();
                    continue;
                }
                Command::Model(choice, key) => {
                    match choice.connect(key.as_deref()) {
                        Ok(model) => {
                            self.model = model;
                            self.params = choice.request_params();
                            self.max_tokens = choice.max_tokens();
                            self.vision = choice.vision != Some(false);
                            self.pictures_in_results = choice.provider != Provider::Ollama;
                            self.context = choice.context();
                            self.forget_reasoning();
                        }
                        Err(error) => on_event(Event::Failed(Problem::Other(error))),
                    }
                    continue;
                }
                Command::Results(results) => {
                    let mut pictures = Vec::new();
                    let mut content: Vec<UserContent> = results
                        .into_iter()
                        .filter_map(|result| {
                            let call = self.waiting.remove(&result.id)?;
                            let mut result = result;
                            // Pictures that cannot go in a tool result follow it.
                            if self.vision && !self.pictures_in_results {
                                for part in &mut result.content {
                                    if let Content::Png(png) = part {
                                        pictures.push(std::mem::take(png));
                                        *part = Content::Text(
                                            "(The picture is in the message after the tool \
                                             results.)"
                                                .to_owned(),
                                        );
                                    }
                                }
                            }
                            Some(UserContent::tool_result(
                                call.id,
                                call.function.name,
                                tool_content(result, self.vision),
                            ))
                        })
                        .collect();
                    if content.is_empty() {
                        continue;
                    }
                    if !pictures.is_empty() {
                        content.push(UserContent::text("The pictures the tools gave:"));
                        content.extend(
                            pictures
                                .into_iter()
                                .map(|png| UserContent::Image(png_image(png))),
                        );
                    }
                    self.history.push(Message::User { content });
                }
            }
            tokio::select! {
                () = self.turn(&on_event) => {}
                () = stop.notified() => on_event(Event::Stopped),
            }
        }
    }

    /// Tells the model the tool calls it is still waiting for were
    /// stopped, as every call needs a result before the next message.
    fn answer_waiting(&mut self) {
        if self.waiting.is_empty() {
            return;
        }
        let content: Vec<UserContent> = self
            .waiting
            .drain()
            .map(|(_, call)| {
                UserContent::tool_result(
                    call.id,
                    call.function.name,
                    vec![ToolResultContent::text("The user stopped this.")],
                )
            })
            .collect();
        self.history.push(Message::User { content });
    }

    /// With `PREV_ASSIST_LOG` naming a file, adds a line of JSON to it for
    /// each request: what went in, the tokens it took and how many came
    /// from the provider's cache, how long it took and the tools it called,
    /// or whether it was for a summary, to compare one change to the
    /// assistant with another.
    fn log(
        &self,
        usage: &rig_core::completion::Usage,
        took: std::time::Duration,
        calls: &[rig_core::completion::message::ToolCall],
        summary: bool,
    ) {
        use std::io::Write;
        let Some(path) = std::env::var_os("PREV_ASSIST_LOG") else {
            return;
        };
        let line = serde_json::json!({
            "instructions": self.instructions.len(),
            "tools": serde_json::to_string(&self.tools).map_or(0, |tools| tools.len()),
            "messages": self.history.len(),
            "input_tokens": usage.input_tokens,
            "cached_tokens": usage.cached_input_tokens,
            "cache_written_tokens": usage.cache_creation_input_tokens,
            "output_tokens": usage.output_tokens,
            "seconds": (took.as_secs_f64() * 10.0).round() / 10.0,
            "calls": calls.iter().map(|call| call.function.name.to_string()).collect::<Vec<_>>(),
            "summary": summary,
        });
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            let _ = writeln!(file, "{line}");
        }
    }

    /// Drops the last model's reasoning, which only it can read, when
    /// another model takes over.
    fn forget_reasoning(&mut self) {
        self.history.retain_mut(|message| {
            let Message::Assistant { content, .. } = message else {
                return true;
            };
            content.retain(|part| !matches!(part, AssistantContent::Reasoning(_)));
            !content.is_empty()
        });
    }

    /// Asks the model to go on from the history, telling plaa what it says.
    async fn turn(&mut self, on_event: &impl Fn(Event)) {
        let mut shortened = false;
        loop {
            match self.attempt(on_event).await {
                Ok(()) => return,
                // Once, with the old results left out, if there are any.
                Err(Problem::ContextTooSmall)
                    if !shortened && shorten_old_results(&mut self.history, 1) =>
                {
                    shortened = true;
                    on_event(Event::Shortened);
                }
                Err(problem) => {
                    on_event(Event::Failed(problem));
                    return;
                }
            }
        }
    }

    /// One request for the turn, telling plaa what the model says.
    async fn attempt(&mut self, on_event: &impl Fn(Event)) -> Result<(), Problem> {
        let Some((last, earlier)) = self.history.split_last() else {
            return Ok(());
        };
        let mut request = CompletionRequest::new(last.clone())
            .messages(earlier.to_vec())
            .preamble(self.instructions.clone())
            .tools(self.tools.clone());
        if let Some(params) = &self.params {
            request = request.additional_params(params.clone());
        }
        if let Some(tokens) = self.max_tokens {
            request = request.max_tokens(tokens);
        }
        let started = std::time::Instant::now();
        let mut stream = self
            .model
            .stream(request)
            .map_err(|error| Problem::of(&error))?;
        while let Some(item) = stream.next().await {
            match item.map_err(|error| Problem::of(&error))? {
                Item::Event(StreamEvent::Text { text, .. }) => on_event(Event::Text(text)),
                Item::Event(StreamEvent::Reasoning { text, .. }) => {
                    on_event(Event::Reasoning(text));
                }
                _ => {}
            }
        }
        let response = stream.finish().await.map_err(|error| Problem::of(&error))?;
        let calls: Vec<rig_core::completion::message::ToolCall> =
            response.tool_calls().cloned().collect();
        self.log(&response.usage, started.elapsed(), &calls, false);
        self.last_input = response.usage.input_tokens.unwrap_or(0);
        self.history.push(Message::Assistant {
            id: None,
            content: response.choice.clone(),
        });
        if calls.is_empty() {
            on_event(Event::Done);
            return Ok(());
        }
        let asked = calls
            .into_iter()
            .enumerate()
            .map(|(index, call)| {
                let id = format!("call-{}-{index}", self.history.len());
                let asked = ToolCall {
                    id: id.clone(),
                    name: call.function.name.to_string(),
                    arguments: call.function.arguments.clone(),
                };
                self.waiting.insert(id, call);
                asked
            })
            .collect();
        on_event(Event::ToolCalls(asked));
        Ok(())
    }

    /// Whether the last request took more than [`COMPACT_AT`] of the
    /// model's context, where it is known.
    fn nearly_full(&self) -> bool {
        self.context
            .is_some_and(|context| self.last_input as f64 > context as f64 * COMPACT_AT)
    }

    /// Has the model summarize the conversation before its last turn, and
    /// puts the summary in its place, so the chat goes on within the
    /// model's context; the last turn stays whole, for what the user says
    /// next to follow from it. Leaves out the old long results instead if
    /// no summary comes.
    async fn compact(&mut self, on_event: &impl Fn(Event)) {
        let Some(boundary) = turn_starts(&self.history)
            .last()
            .copied()
            .filter(|&at| at > 0)
        else {
            return;
        };
        on_event(Event::Compacting);
        // The same tools and instructions, and the conversation as it was,
        // so the provider reads them from its cache.
        let mut request = CompletionRequest::new(Message::user(COMPACT_PROMPT))
            .messages(self.history[..boundary].to_vec())
            .preamble(self.instructions.clone())
            .tools(self.tools.clone());
        if let Some(params) = &self.params {
            request = request.additional_params(params.clone());
        }
        if let Some(tokens) = self.max_tokens {
            request = request.max_tokens(tokens);
        }
        let started = std::time::Instant::now();
        let summary = match self.model.call(request).await {
            Ok(response) => {
                self.log(&response.usage, started.elapsed(), &[], true);
                text_of(&response.choice)
            }
            Err(_) => String::new(),
        };
        if summary.trim().is_empty() {
            if shorten_old_results(&mut self.history, 1) {
                on_event(Event::Shortened);
            }
            return;
        }
        let mut kept = self.history.split_off(boundary);
        if let Some(Message::User { content }) = kept.first_mut() {
            content.insert(
                0,
                UserContent::text(format!(
                    "(Our conversation before this was too long to keep, so here is your \
                     summary of it.)\n\n{}",
                    summary.trim()
                )),
            );
        }
        self.history = kept;
        // Not known again until the next request.
        self.last_input = 0;
        on_event(Event::Compacted);
    }
}

/// The share of the model's context the conversation may take before it
/// is summarized.
const COMPACT_AT: f64 = 0.8;

/// What the model is asked for when the conversation is summarized.
const COMPACT_PROMPT: &str = "Our conversation is getting too long for you to read at once, so \
    it will be replaced by your summary of it. Write that summary now, for yourself to go on \
    from: what I asked and what you answered, the pages, figures and short quotes you found, \
    the ids of the annotations you made and what they are, and anything left to do. Leave out \
    tool output that can be read again, and say where it came from instead. Reply with the \
    summary alone, and call no tools.";

/// Where each turn starts: each thing the user said, rather than tool
/// results.
fn turn_starts(history: &[Message]) -> Vec<usize> {
    (0..history.len())
        .filter(|&index| {
            matches!(&history[index], Message::User { content }
                if !content.iter().any(|part| matches!(part, UserContent::ToolResult(_))))
        })
        .collect()
}

/// Leaves out the long tool results and pictures from before the last
/// `keep` turns, so the chat fits the model's context: the model can ask
/// for them again. Returns whether anything was left out.
fn shorten_old_results(history: &mut [Message], keep: usize) -> bool {
    let starts = turn_starts(history);
    let Some(&boundary) = starts
        .len()
        .checked_sub(keep)
        .and_then(|first| starts.get(first))
    else {
        return false;
    };
    let mut shortened = false;
    for message in &mut history[..boundary] {
        let Message::User { content } = message else {
            continue;
        };
        for part in content.iter_mut() {
            match part {
                UserContent::ToolResult(result) if long(&result.content) => {
                    result.content = vec![ToolResultContent::text(format!(
                        "(This {} result was left out to make room; call the tool again if \
                         it is needed.)",
                        result.name
                    ))];
                    shortened = true;
                }
                UserContent::Image(_) => {
                    *part = UserContent::text("(A picture, left out to make room.)");
                    shortened = true;
                }
                _ => {}
            }
        }
    }
    shortened
}

/// The longest tool result, in bytes of text, kept when old ones are left
/// out: ids, counts and short answers stay.
const OLD_RESULT_LIMIT: usize = 600;

/// Whether a tool result is worth leaving out once it is old: a picture,
/// or more text than [`OLD_RESULT_LIMIT`].
fn long(content: &[ToolResultContent]) -> bool {
    let mut length = 0;
    for part in content {
        match part {
            ToolResultContent::Text(text) => length += text.text().len(),
            ToolResultContent::Image(_) => return true,
            other => length += serde_json::to_string(other).map_or(0, |json| json.len()),
        }
    }
    length > OLD_RESULT_LIMIT
}

/// A PNG as rig sends it on.
fn png_image(png: Vec<u8>) -> Image {
    Image {
        data: DocumentSourceKind::Base64(base64::engine::general_purpose::STANDARD.encode(png)),
        media_type: Some(ImageMediaType::PNG),
        detail: None,
        additional_params: None,
    }
}

/// A tool's result as rig sends it on.
fn tool_content(result: ToolResult, vision: bool) -> Vec<ToolResultContent> {
    let mut content: Vec<ToolResultContent> = result
        .content
        .into_iter()
        .map(|part| match part {
            Content::Text(text) => ToolResultContent::text(text),
            Content::Png(_) if !vision => ToolResultContent::text(
                "(A picture, left out: this model cannot see pictures. Read the text with \
                 page_text or search instead.)",
            ),
            Content::Png(png) => ToolResultContent::Image(png_image(png)),
        })
        .collect();
    if content.is_empty() {
        content.push(ToolResultContent::text(if result.failed {
            "The tool failed."
        } else {
            "Done."
        }));
    }
    content
}

/// Assistant content that is plain text.
fn text_of(content: &[AssistantContent]) -> String {
    content
        .iter()
        .filter_map(|part| match part {
            AssistantContent::Text(text) => Some(text.text()),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choices_round_trip_through_settings() {
        let choice = ModelChoice {
            provider: Provider::Ollama,
            model: "qwen3.8".to_owned(),
            address: Some("http://localhost:11434".to_owned()),
            context: None,
            vision: None,
        };
        let text = serde_json::to_string(&choice).unwrap();
        assert!(text.contains("\"ollama\""), "{text}");
        assert_eq!(serde_json::from_str::<ModelChoice>(&text).unwrap(), choice);
    }

    #[test]
    fn cloud_models_need_a_key_and_a_name() {
        let choice = |model: &str| ModelChoice {
            provider: Provider::Anthropic,
            model: model.to_owned(),
            address: None,
            context: None,
            vision: None,
        };
        assert!(choice("claude").connect(None).is_err());
        assert!(choice("").connect(Some("key")).is_err());
        assert!(choice("claude").connect(Some("key")).is_ok());
    }

    #[test]
    fn only_ollama_gets_a_context_size() {
        let mut choice = ModelChoice {
            provider: Provider::Ollama,
            model: "qwen3.8".to_owned(),
            address: None,
            context: None,
            vision: None,
        };
        assert_eq!(
            choice.request_params(),
            Some(serde_json::json!({ "num_ctx": DEFAULT_CONTEXT }))
        );
        choice.context = Some(131_072);
        assert_eq!(
            choice.request_params(),
            Some(serde_json::json!({ "num_ctx": 131_072 }))
        );
        choice.provider = Provider::OpenAiCompatible;
        assert_eq!(choice.request_params(), None);
    }

    #[test]
    fn provider_errors_become_plain_problems() {
        use http::StatusCode;
        use rig_core::ProviderResponseError;
        let reply = |status: u16, body: &str| {
            Problem::of(&ProviderError::ProviderResponse(
                ProviderResponseError::new(StatusCode::from_u16(status).unwrap(), body),
            ))
        };
        // What Ollama says.
        let ollama = r#"{"error":"{\"error\":{\"code\":400,\"message\":\"request (9177 tokens) exceeds the available context size (4096 tokens), try increasing it\",\"type\":\"exceed_context_size_error\"}}"}"#;
        assert_eq!(reply(400, ollama), Problem::ContextTooSmall);
        assert_eq!(
            reply(
                400,
                r#"{"type":"error","error":{"type":"invalid_request_error","message":"prompt is too long: 210000 tokens > 200000 maximum"}}"#
            ),
            Problem::ContextTooSmall
        );
        assert_eq!(reply(401, "{}"), Problem::Key);
        let limited = r#"{"error":{"message":"Rate limit reached for gpt-6.1-sol on requests per min (RPM): Limit 3, Used 3. Please try again in 20s.","type":"requests","code":"rate_limit_exceeded"}}"#;
        assert_eq!(
            reply(429, limited),
            Problem::RateLimited(
                "Rate limit reached for gpt-6.1-sol on requests per min (RPM): Limit 3, Used 3. \
                 Please try again in 20s."
                    .to_owned()
            )
        );
        // OpenAI's new accounts, before any credit is bought.
        let quota = r#"{"error":{"message":"You exceeded your current quota, please check your plan and billing details.","type":"insufficient_quota","code":"insufficient_quota"}}"#;
        assert_eq!(reply(429, quota), Problem::NoCredit);
        let anthropic = r#"{"type":"error","error":{"type":"invalid_request_error","message":"Your credit balance is too low to access the Anthropic API."}}"#;
        assert_eq!(reply(400, anthropic), Problem::NoCredit);
        assert_eq!(
            reply(404, r#"{"error":"model 'qwen9' not found"}"#),
            Problem::NoSuchModel
        );
        assert_eq!(reply(529, "{}"), Problem::Unavailable);
        assert_eq!(
            reply(400, r#"{"error":{"message":"Bad tool schema"}}"#),
            Problem::Other("Bad tool schema".to_owned())
        );
        assert_eq!(
            reply(400, r#"{"error":"{\"error\":{\"message\":\"Nested\"}}"}"#),
            Problem::Other("Nested".to_owned())
        );
        assert_eq!(
            reply(400, "plain words"),
            Problem::Other("plain words".to_owned())
        );
    }

    #[test]
    fn old_long_results_are_left_out() {
        let result = |text: &str| {
            UserContent::tool_result(
                rig_core::completion::message::LocalCallId::new().into(),
                "page_text".try_into().expect("a tool name"),
                vec![ToolResultContent::text(text)],
            )
        };
        let long_text = "word ".repeat(200);
        let mut history = vec![
            Message::user("What is on page 3?"),
            Message::User {
                content: vec![result(&long_text), result("Done.")],
            },
            Message::user("And page 4?"),
            Message::User {
                content: vec![result(&long_text)],
            },
        ];
        assert_eq!(turn_starts(&history), vec![0, 2]);
        // With both turns kept, nothing goes.
        let whole = format!("{history:?}");
        assert!(!shorten_old_results(&mut history, 2));
        assert_eq!(format!("{history:?}"), whole);
        assert!(shorten_old_results(&mut history, 1));
        let texts: Vec<String> = history
            .iter()
            .flat_map(|message| match message {
                Message::User { content } => content
                    .iter()
                    .filter_map(|part| match part {
                        UserContent::ToolResult(result) => match &result.content[0] {
                            ToolResultContent::Text(text) => Some(text.text().to_owned()),
                            _ => None,
                        },
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            })
            .collect();
        // The first turn's long result goes, its short one stays, and the
        // last turn's is kept whole.
        assert!(texts[0].starts_with("(This page_text result was left out"));
        assert_eq!(texts[1], "Done.");
        assert_eq!(texts[2], long_text);
        // Nothing is left to leave out the second time.
        assert!(!shorten_old_results(&mut history, 1));
    }

    #[test]
    fn only_a_known_context_is_counted() {
        let choice = |provider, context| ModelChoice {
            provider,
            model: "m".to_owned(),
            address: None,
            context,
            vision: None,
        };
        assert_eq!(
            choice(Provider::Ollama, None).context(),
            Some(u64::from(DEFAULT_CONTEXT))
        );
        assert_eq!(choice(Provider::Anthropic, None).context(), None);
        assert_eq!(choice(Provider::OpenAi, Some(9_000)).context(), Some(9_000));
    }

    #[test]
    fn tool_results_keep_text_and_pictures() {
        let result = ToolResult {
            id: "a".to_owned(),
            content: vec![Content::Text("hi".to_owned()), Content::Png(vec![1, 2, 3])],
            failed: false,
        };
        let content = tool_content(result.clone(), true);
        assert_eq!(content.len(), 2);
        assert!(matches!(&content[1], ToolResultContent::Image(_)));
        // A model that cannot see gets a line in the picture's place.
        let content = tool_content(result, false);
        assert_eq!(content.len(), 2);
        assert!(!matches!(&content[1], ToolResultContent::Image(_)));
        let failed = ToolResult {
            id: "b".to_owned(),
            content: Vec::new(),
            failed: true,
        };
        assert_eq!(tool_content(failed, true).len(), 1);
    }
}
