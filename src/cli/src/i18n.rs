// Copyright 2026 Michael F. Collins, III
// Licensed under the Naked Time license. See LICENSE.md for details.

//! Localization of user-facing text with Project Fluent.

use std::borrow::Cow;
use std::sync::LazyLock;

use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use i18n_embed::{DesktopLanguageRequester, LanguageLoader};
use rust_embed::RustEmbed;
use unic_langid::LanguageIdentifier;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader = fluent_language_loader!();
    // Load the fallback language so messages always resolve.
    let _ = loader.load_fallback_language(&Localizations);
    // Fluent otherwise wraps placeables in invisible bidi isolation marks,
    // which some terminals render as garbage.
    loader.set_use_isolating(false);
    loader
});

/// Looks up a localized message, e.g. `fl!("app-about")` or
/// `fl!("error-prefix", message = text)`. IDs and arguments are checked at
/// compile time against the fallback `.ftl` file.
macro_rules! fl {
    ($($args:tt)*) => {
        ::i18n_embed_fl::fl!($crate::i18n::LOADER, $($args)*)
    };
}
pub(crate) use fl;

/// Selects the language from `NAKED_TIME_LANG` or the operating system's
/// settings, falling back to en-US. Set `NAKED_TIME_PSEUDO=1` to
/// pseudo-localize all text, which makes hard-coded strings easy to spot.
pub fn init() {
    let requested: Vec<LanguageIdentifier> = match std::env::var("NAKED_TIME_LANG")
        .ok()
        .and_then(|lang| lang.parse().ok())
    {
        Some(lang) => vec![lang],
        None => DesktopLanguageRequester::requested_languages(),
    };
    let _ = i18n_embed::select(&*LOADER, &Localizations, &requested);
    // Selecting a language rebuilds the bundles, so set this again.
    LOADER.set_use_isolating(false);

    if std::env::var_os("NAKED_TIME_PSEUDO").is_some_and(|v| v == "1") {
        enable_pseudo();
    }
}

fn pseudo(text: &str) -> Cow<'_, str> {
    fluent_pseudo::transform(text, false, true)
}

fn enable_pseudo() {
    LOADER.with_bundles_mut(|bundle| bundle.set_transform(Some(pseudo)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_messages_resolve() {
        assert_eq!(fl!("help-flag-help"), "Print help");
    }

    #[test]
    fn arguments_are_substituted() {
        assert_eq!(
            fl!("cmd-pomodoro-start-about", minutes = 25),
            "Start a 25-minute Pomodoro and record it"
        );
    }

    #[test]
    fn pseudo_transform_changes_text() {
        assert_ne!(pseudo("Print help"), "Print help");
    }
}
