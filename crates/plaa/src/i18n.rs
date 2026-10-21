//! The interface's text in the user's language.
//!
//! Every string plaa shows lives in `i18n/<language>/plaa.ftl` at the top
//! of the repository, in Project Fluent's format. `i18n/en` is the source:
//! the `fl!` macro checks each key against it when plaa is compiled, and
//! other languages fall back to it for keys they lack. Adding a language
//! is adding its folder; no code changes.

use std::collections::BTreeSet;
use std::sync::{LazyLock, Mutex, PoisonError, RwLock};

use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use i18n_embed::unic_langid::{CharacterDirection, LanguageIdentifier};
use i18n_embed::{DesktopLanguageRequester, LanguageLoader};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../i18n/"]
struct Localizations;

/// The loaded text, in the first language the user asks for that plaa
/// has, else English.
pub static LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader = fluent_language_loader!();
    select(&loader);
    loader
});

/// The language chosen in Settings; none follows the system.
static CHOSEN: RwLock<Option<LanguageIdentifier>> = RwLock::new(None);

fn select(loader: &FluentLanguageLoader) {
    let _ = i18n_embed::select(loader, &Localizations, &requested_languages());
    // Values placed in a sentence are wrapped in direction marks so a
    // Latin file name keeps its order inside right to left text; left to
    // right languages need no marks.
    let right_to_left = loader.current_language().character_direction() == CharacterDirection::RTL;
    loader.set_use_isolating(right_to_left);
    scramble_ui::dir::set_right_to_left(right_to_left);
}

/// The languages the user asks for, most wanted first: `PLAA_LANG` (a
/// language tag such as `he` or `pt-BR`, for trying translations), then
/// the one chosen in Settings, then the system's.
fn requested_languages() -> Vec<LanguageIdentifier> {
    if cfg!(test) {
        return vec!["en".parse().expect("a language tag")];
    }
    let chosen = CHOSEN
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    let mut languages: Vec<LanguageIdentifier> = std::env::var("PLAA_LANG")
        .ok()
        .and_then(|tag| tag.parse().ok())
        .into_iter()
        .chain(chosen)
        .collect();
    languages.extend(DesktopLanguageRequester::requested_languages());
    with_aliases(languages)
}

/// The languages asked for, each followed by the one plaa's text is under
/// where the two differ: Norwegian (`no`) by Bokmål (`nb`), the written
/// standard most use, as systems ask for either.
fn with_aliases(languages: Vec<LanguageIdentifier>) -> Vec<LanguageIdentifier> {
    let mut out = Vec::with_capacity(languages.len());
    for language in languages {
        let alias = (language.language.as_str() == "no").then(|| {
            let mut bokmal = language.clone();
            bokmal.language = "nb".parse().expect("a language subtag");
            bokmal
        });
        out.push(language);
        out.extend(alias);
    }
    out
}

/// Shows the interface in the language tagged `tag`, or the system's for
/// `None` or a tag plaa has no text for.
pub fn set_language(tag: Option<&str>) {
    *CHOSEN.write().unwrap_or_else(PoisonError::into_inner) = tag.and_then(|tag| tag.parse().ok());
    select(&LOADER);
    // scramble-ui's components name their own buttons.
    scramble_ui::labels::set(scramble_ui::labels::Labels {
        close: crate::fl!("common-close"),
        more: crate::fl!("common-more"),
        keep_toolbar_shown: crate::fl!("overlay-keep-shown"),
        auto_hide_toolbar: crate::fl!("overlay-auto-hide"),
    });
}

/// The languages plaa has text for: each one's tag and its name in its
/// own language, sorted by tag.
pub fn languages() -> &'static [(String, String)] {
    static LANGUAGES: LazyLock<Vec<(String, String)>> = LazyLock::new(|| {
        let mut languages: Vec<(String, String)> = LOADER
            .available_languages(&Localizations)
            .unwrap_or_default()
            .into_iter()
            .map(|language| (language.to_string(), name_of(&language)))
            .collect();
        languages.sort();
        languages
    });
    &LANGUAGES
}

fn name_of(language: &LanguageIdentifier) -> String {
    let loader = fluent_language_loader!();
    let _ = loader.load_languages(&Localizations, std::slice::from_ref(language));
    loader.get("language-name")
}

/// Text that must outlive the view showing it, such as a tab label iced
/// borrows. Each distinct text is kept once for the life of plaa.
pub fn lasting(text: String) -> &'static str {
    static KEPT: Mutex<BTreeSet<&'static str>> = Mutex::new(BTreeSet::new());
    let mut kept = KEPT.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(found) = kept.get(text.as_str()) {
        return found;
    }
    let text: &'static str = Box::leak(text.into_boxed_str());
    kept.insert(text);
    text
}

/// `fl!("key")` or `fl!("key", name = value)`: the text for `key` in the
/// interface's language. Checked against the English file at compile time.
#[macro_export]
macro_rules! fl {
    ($key:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LOADER, $key)
    }};
    ($key:literal, $($args:tt)*) => {{
        i18n_embed_fl::fl!($crate::i18n::LOADER, $key, $($args)*)
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::{Path, PathBuf};

    use super::*;

    #[test]
    fn english_loads() {
        assert_eq!(crate::fl!("app-name"), "plaa");
        assert!(super::languages().iter().any(|(tag, _)| tag == "en"));
    }

    #[test]
    fn norwegian_asks_for_bokmal() {
        let asked: Vec<LanguageIdentifier> = vec!["no-NO".parse().unwrap(), "en".parse().unwrap()];
        let tags: Vec<String> = with_aliases(asked)
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(tags, ["no-NO", "nb-NO", "en"]);
    }

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../i18n")
    }

    fn folders() -> Vec<String> {
        let mut languages: Vec<String> = std::fs::read_dir(root())
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().join("plaa.ftl").is_file())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        languages.sort();
        languages
    }

    /// Each message's key and the `{ $variables }` its text uses. Keys
    /// start a line; attributes (`.name =`) and continuation lines belong
    /// to the message above.
    fn messages(language: &str) -> BTreeMap<String, BTreeSet<String>> {
        let path = root().join(language).join("plaa.ftl");
        let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("reading {path:?}"));
        let mut messages: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut current: Option<String> = None;
        for line in text.lines() {
            let starts_message =
                line.split_once('=')
                    .map(|(key, _)| key.trim_end())
                    .filter(|key| {
                        !line.starts_with([' ', '\t', '#', '.'])
                            && key.chars().next().is_some_and(|c| c.is_ascii_lowercase())
                            && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
                    });
            if let Some(key) = starts_message {
                assert!(
                    !messages.contains_key(key),
                    "{language}: `{key}` is defined twice"
                );
                messages.insert(key.to_owned(), BTreeSet::new());
                current = Some(key.to_owned());
            } else if line.is_empty() || line.starts_with('#') {
                current = None;
            }
            if let Some(key) = &current {
                let variables = messages.get_mut(key).unwrap();
                for piece in line.split('$').skip(1) {
                    let name: String = piece
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                        .collect();
                    if !name.is_empty() {
                        variables.insert(name);
                    }
                }
            }
        }
        messages
    }

    #[test]
    fn every_language_loads() {
        for language in folders() {
            let id: LanguageIdentifier = language.parse().expect("folder names are language tags");
            let loader = fluent_language_loader!();
            loader
                .load_languages(&Localizations, &[id])
                .unwrap_or_else(|error| panic!("{language}: {error}"));
        }
    }

    #[test]
    fn translations_match_english() {
        let english = messages("en");
        assert!(!english.is_empty());
        for language in folders().into_iter().filter(|language| language != "en") {
            let translated = messages(&language);
            assert!(
                translated.contains_key("language-name"),
                "{language}: `language-name` names the language in the Settings list"
            );
            for (key, variables) in &translated {
                let Some(expected) = english.get(key) else {
                    panic!("{language}: `{key}` is not in the English file");
                };
                let unknown: Vec<_> = variables.difference(expected).collect();
                assert!(
                    unknown.is_empty(),
                    "{language}: `{key}` uses {unknown:?}, which English does not provide"
                );
            }
            let missing = english
                .keys()
                .filter(|key| !translated.contains_key(*key))
                .count();
            if missing > 0 {
                println!(
                    "{language}: {missing} of {} keys not translated yet",
                    english.len()
                );
            }
        }
    }
}
