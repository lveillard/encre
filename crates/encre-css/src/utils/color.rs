//! Define some color manipulation functions.
use crate::config::{BUILTIN_COLORS, Config};

use std::borrow::Cow;

/// Returns whether the modifier is matching a builtin color. The builtin colors are:
///
/// - `current`, `inherit`, `transparent`, `black`, `white`;
/// - Any key contained in the [`BUILTIN_COLORS`] list.
pub fn is_matching_builtin_color(config: &Config, mut modifier: &str) -> bool {
    if ["current", "inherit", "transparent"].contains(&modifier) {
        return true;
    }

    // Trim the opacity suffix, if present
    if let Some((new_modifier, opacity)) = modifier.split_once('/') {
        if opacity.parse::<usize>().is_err() {
            return false;
        }

        modifier = new_modifier;
    }

    if ["black", "white"].contains(&modifier) {
        return true;
    }

    BUILTIN_COLORS.contains_key(modifier) || config.theme.colors.contains(modifier)
}

/// Get a color from a modifier.
///
/// The third argument is used to set the opacity type used:
///
/// - [`Option::None`] won't use opacity (except if the opacity syntax is used, for example in `bg-red-500/25`);
/// - [`Option::Some`] contains a variable which will be added as the opacity of the color, used to
///   dynamically change the opacity.
pub fn get<'a>(config: &'a Config, modifier: &'a str) -> Option<Cow<'a, str>> {
    // Handle the new opacity syntax (e.g. `bg-red-500/25`)
    let (opacity, modifier) = {
        // The `current` and `inherit` modifiers have static values
        if modifier == "current" {
            return Some(Cow::Borrowed("currentColor"));
        } else if modifier == "transparent" {
            return Some(Cow::Borrowed("transparent"));
        } else if modifier == "inherit" {
            return Some(Cow::Borrowed("inherit"));
        } else if let Some((new_modifier, opacity_suffix)) = modifier.split_once('/') {
            if let Ok(opacity_number) = opacity_suffix.parse::<usize>() {
                (Some(Cow::Owned(format!("{opacity_number}%"))), new_modifier)
            } else {
                let opacity_var = opacity_suffix
                    .strip_prefix("[")
                    .and_then(|w| w.strip_suffix("]"))?;
                (Some(Cow::Borrowed(opacity_var)), new_modifier)
            }
        } else {
            (None, modifier)
        }
    };

    let color_result = if let Some(color) = config.theme.colors.get(modifier) {
        // Custom theme values override builtin colors
        Some(&**color)
    } else {
        BUILTIN_COLORS.get(modifier).copied()
    }?;

    // Add the opacity effect if needed
    let with_opacity_result = if let Some(opacity) = opacity {
        Cow::Owned(format!(
            "color-mix(in oklab, {color_result} {opacity}, transparent)"
        ))
    } else {
        Cow::Borrowed(color_result)
    };

    Some(with_opacity_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Config;

    #[test]
    fn get_color() {
        let mut config = Config::default();
        config.theme.colors.add("my-rosa", "#f23cff");
        config
            .theme
            .colors
            .add("my-indigo", "oklch(0.6667 0.1727 294.14)");

        assert_eq!(
            get(&config, "red-400/[var(--opacity)]").unwrap(),
            "color-mix(in oklab, oklch(70.4% .191 22.216) var(--opacity), transparent)"
        );
        assert_eq!(
            get(&config, "red-400/20").unwrap(),
            "color-mix(in oklab, oklch(70.4% .191 22.216) 20%, transparent)"
        );
        assert_eq!(get(&config, "red-400").unwrap(), "oklch(70.4% .191 22.216)");
        assert!(get(&config, "red-400/aaa").is_none());
        assert_eq!(get(&config, "current").unwrap(), "currentColor");
        assert_eq!(get(&config, "inherit").unwrap(), "inherit");
        assert_eq!(get(&config, "transparent").unwrap(), "transparent");
        assert_eq!(get(&config, "my-rosa").unwrap(), "#f23cff");
        assert_eq!(
            get(&config, "my-rosa/20").unwrap(),
            "color-mix(in oklab, #f23cff 20%, transparent)"
        );
        assert_eq!(
            get(&config, "my-indigo").unwrap(),
            "oklch(0.6667 0.1727 294.14)"
        );
        assert_eq!(
            get(&config, "my-indigo/20").unwrap(),
            "color-mix(in oklab, oklch(0.6667 0.1727 294.14) 20%, transparent)"
        );

        // NOTE: when using a variable as an opacity, the variable type must be <percentage>
        assert_eq!(
            get(&config, "my-indigo/[var(--my-opacity)]").unwrap(),
            "color-mix(in oklab, oklch(0.6667 0.1727 294.14) var(--my-opacity), transparent)"
        );
    }
}
