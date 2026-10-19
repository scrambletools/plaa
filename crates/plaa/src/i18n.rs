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
    languages
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
    #[test]
    fn english_loads() {
        assert_eq!(crate::fl!("app-name"), "plaa");
        assert_eq!(
            super::languages().first().map(|(tag, _)| tag.as_str()),
            Some("en")
        );
    }
}
