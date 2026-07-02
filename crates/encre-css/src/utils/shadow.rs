//! Shadow parsing utility functions.
use super::value_matchers::{is_matching_color, is_matching_length, is_matching_var};

use std::{borrow::Cow, fmt};

const SHADOW_KEYWORDS: [&str; 5] = ["none", "inherit", "initial", "revert", "unset"];

#[derive(Debug, PartialEq)]
enum Shadow<'a> {
    Raw([&'a str; 6]),
    Keyword(&'a str),
    Variable(&'a str),
    Shorthand1 {
        is_inset: bool,
        offset_x: &'a str,
        offset_y: &'a str,
        color: Cow<'a, str>,
    },
    Shorthand2 {
        is_inset: bool,
        offset_x: &'a str,
        offset_y: &'a str,
        blur_radius: &'a str,
        color: Cow<'a, str>,
    },
    Full {
        is_inset: bool,
        offset_x: &'a str,
        offset_y: &'a str,
        blur_radius: &'a str,
        spread_radius: &'a str,
        color: Cow<'a, str>,
    },
}

impl Shadow<'_> {
    fn new_raw() -> Self {
        Self::Raw([""; 6])
    }

    /// Parse a real [`Shadow`] from a [`Shadow::Raw`] variant.
    fn parse(&self) -> Option<Self> {
        if let Shadow::Raw(shadow) = self {
            // Handle inset shadows
            let (is_inset, shadow) = if shadow[0].is_empty() {
                (false, &shadow[..])
            } else if shadow[0] == "inset" {
                (true, &shadow[1..])
            } else {
                (false, &shadow[..])
            };

            // Check the number of parts
            if shadow.is_empty()
                && ((is_inset && shadow.len() > 6) || (!is_inset && shadow.len() > 5))
            {
                return None;
            }

            let len = shadow.iter().position(|p| p.is_empty()).unwrap_or(5);
            if len == 1 {
                // Keyword value
                if SHADOW_KEYWORDS.contains(&shadow[0]) {
                    Some(Shadow::Keyword(shadow[0]))
                } else if is_matching_var(shadow[0]) {
                    Some(Shadow::Variable(shadow[0]))
                } else {
                    None
                }
            } else if len == 3 {
                // Shorthand 1: offset-x | offset-y | color
                if is_matching_length(shadow[0])
                    && is_matching_length(shadow[1])
                    && is_matching_color(shadow[2])
                {
                    Some(Shadow::Shorthand1 {
                        is_inset,
                        offset_x: shadow[0],
                        offset_y: shadow[1],
                        color: Cow::Borrowed(shadow[2]),
                    })
                } else {
                    None
                }
            } else if len == 4 {
                // Shorthand 2: offset-x | offset-y | blur-radius | color
                if is_matching_length(shadow[0])
                    && is_matching_length(shadow[1])
                    && is_matching_length(shadow[2])
                    && is_matching_color(shadow[3])
                {
                    Some(Shadow::Shorthand2 {
                        is_inset,
                        offset_x: shadow[0],
                        offset_y: shadow[1],
                        blur_radius: shadow[2],
                        color: Cow::Borrowed(shadow[3]),
                    })
                } else {
                    None
                }
            } else if len == 5 {
                // Full: offset-x | offset-y | blur-radius | spread-radius | color
                if is_matching_length(shadow[0])
                    && is_matching_length(shadow[1])
                    && is_matching_length(shadow[2])
                    && is_matching_length(shadow[3])
                    && is_matching_color(shadow[4])
                {
                    Some(Shadow::Full {
                        is_inset,
                        offset_x: shadow[0],
                        offset_y: shadow[1],
                        blur_radius: shadow[2],
                        spread_radius: shadow[3],
                        color: Cow::Borrowed(shadow[4]),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl fmt::Display for Shadow<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shadow::Raw(s) => write!(f, "{}", s.join(" ")),
            Shadow::Keyword(keyword) => write!(f, "{keyword}"),
            Shadow::Variable(variable) => write!(f, "{variable}"),
            Shadow::Shorthand1 {
                is_inset,
                offset_x,
                offset_y,
                color,
            } => write!(
                f,
                "{}{offset_x} {offset_y} {color}",
                if *is_inset { "inset " } else { "" }
            ),
            Shadow::Shorthand2 {
                is_inset,
                offset_x,
                offset_y,
                blur_radius,
                color,
            } => write!(
                f,
                "{}{offset_x} {offset_y} {blur_radius} {color}",
                if *is_inset { "inset " } else { "" }
            ),
            Shadow::Full {
                is_inset,
                offset_x,
                offset_y,
                blur_radius,
                spread_radius,
                color,
            } => write!(
                f,
                "{}{offset_x} {offset_y} {blur_radius} {spread_radius} {color}",
                if *is_inset { "inset " } else { "" }
            ),
        }
    }
}

/// A list of shadows, used in the [`box-shadow`](https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow) CSS property.
#[derive(Debug, PartialEq)]
pub struct ShadowList<'a>(Vec<Shadow<'a>>);

impl<'a> ShadowList<'a> {
    /// Parse an arbitrary string into a [`ShadowList`].
    ///
    /// # Example
    ///
    /// ```
    /// use encre_css::utils::shadow::ShadowList;
    /// assert_eq!(ShadowList::parse("10px 20px 30px 40px rgb(12 12 12)").unwrap().to_string(), "10px 20px 30px 40px rgb(12 12 12)".to_string());
    /// assert_eq!(ShadowList::parse("1px 2px 3px 4px"), None);
    /// ```
    pub fn parse(value: &'a str) -> Option<Self> {
        let mut parenthesis_level = 0;
        let mut last_index = 0;
        let mut shadows = vec![Shadow::new_raw()];

        for (ch_index, ch) in value.char_indices() {
            match ch {
                '(' => {
                    parenthesis_level += 1;
                }
                ')' => {
                    parenthesis_level -= 1;
                }
                ' ' if parenthesis_level == 0 => {
                    let Shadow::Raw(shadow) = shadows.last_mut()? else {
                        // Shadow already parsed but a space was encountered
                        return None;
                    };

                    if !value[last_index..ch_index].is_empty() {
                        // Find the index of the first free part
                        let index = shadow.iter().position(|p| p.is_empty()).unwrap_or(5);

                        // Insert the part
                        shadow[index] = &value[last_index..ch_index];
                    }

                    // Update the index (and ignore the space)
                    last_index = ch_index + 1;
                }
                ',' if parenthesis_level == 0 => {
                    // Add the last part (not suffixed by `_`)
                    let Shadow::Raw(shadow) = shadows.last_mut()? else {
                        // Shadow already parsed but a space was encountered
                        return None;
                    };

                    // Find the index of the first free part
                    let index = shadow.iter().position(|p| p.is_empty()).unwrap_or(5);

                    // Insert the part
                    shadow[index] = &value[last_index..ch_index];

                    // Ignore the shadow if it is empty
                    if !shadow.iter().all(|p| p.is_empty()) {
                        // Parse the shadow
                        let parsed_shadow = shadows.last()?.parse()?;
                        *shadows.last_mut()? = parsed_shadow;

                        // Start the next shadow
                        shadows.push(Shadow::new_raw());
                    }

                    // Update the index (and ignore the comma)
                    last_index = ch_index + 1;
                }
                _ => (),
            }
        }

        if value.is_empty() {
            return None;
        }

        // Add the last part (not suffixed by `,`)
        if last_index != value.len() - 1 {
            // Find the index of the first free part
            let Shadow::Raw(shadow) = shadows.last_mut()? else {
                return None;
            };
            let index = shadow.iter().position(|p| p.is_empty()).unwrap_or(5);

            // Insert the part
            shadow[index] = &value[last_index..value.len()];

            // Parse the shadow
            let parsed_shadow = shadows.last()?.parse()?;
            *shadows.last_mut()? = parsed_shadow;
        }

        Some(Self(shadows))
    }

    /// Replace the color of all shadows with the color given as the first argument.
    ///
    /// If the given color contains `{}`, it will be replaced by the old color.
    pub fn replace_all_colors(&mut self, new_color: &'a str) {
        self.0.iter_mut().for_each(|shadow| match shadow {
            Shadow::Shorthand1 { color, .. }
            | Shadow::Shorthand2 { color, .. }
            | Shadow::Full { color, .. } => {
                *color = Cow::Owned(new_color.replace("{}", color))
            }
            _ => (),
        });
    }
}

impl fmt::Display for ShadowList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, v) in self.0.iter().enumerate() {
            write!(f, "{}{}", v, if i == self.0.len() - 1 { "" } else { "," })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_shadow_test() {
        let shadow = "20px 35px 60px -15px rgba(0,0,0,0.3),0 72px rgba(0,2,42,0.2),inset 23px 42em 42px rgba(255,0,0,1)";
        let result = ShadowList::parse(shadow).unwrap();
        assert_eq!(
            result,
            ShadowList(vec![
                Shadow::Full {
                    is_inset: false,
                    offset_x: "20px",
                    offset_y: "35px",
                    blur_radius: "60px",
                    spread_radius: "-15px",
                    color: Cow::Borrowed("rgba(0,0,0,0.3)"),
                },
                Shadow::Shorthand1 {
                    is_inset: false,
                    offset_x: "0",
                    offset_y: "72px",
                    color: Cow::Borrowed("rgba(0,2,42,0.2)"),
                },
                Shadow::Shorthand2 {
                    is_inset: true,
                    offset_x: "23px",
                    offset_y: "42em",
                    blur_radius: "42px",
                    color: Cow::Borrowed("rgba(255,0,0,1)"),
                }
            ])
        );

        assert_eq!(
            ShadowList::parse("var(--a, 0 0 1px rgb(0, 0, 0)),1px 2px 3rem rgb(0, 0, 0)").unwrap(),
            ShadowList(vec![
                Shadow::Variable("var(--a, 0 0 1px rgb(0, 0, 0))"),
                Shadow::Shorthand2 {
                    is_inset: false,
                    offset_x: "1px",
                    offset_y: "2px",
                    blur_radius: "3rem",
                    color: Cow::Borrowed("rgb(0, 0, 0)"),
                },
            ])
        );

        assert_eq!(
            ShadowList::parse("none").unwrap(),
            ShadowList(vec![Shadow::Keyword("none")])
        );

        assert_eq!(
            ShadowList::parse("inset 0 5px 90px 40px rgba(0,0,0,0.2)").unwrap(),
            ShadowList(vec![Shadow::Full {
                is_inset: true,
                offset_x: "0",
                offset_y: "5px",
                blur_radius: "90px",
                spread_radius: "40px",
                color: Cow::Borrowed("rgba(0,0,0,0.2)")
            }])
        );
    }

    #[test]
    fn format_shadow() {
        let shadow = "20px 35px 60px -15px rgba(0,0,0,0.3),0 72px rgba(0,2,42,0.2),inset 23px 42em rgba(255,0,0,1)";
        let result = ShadowList::parse(shadow).unwrap();
        assert_eq!(&result.to_string(), shadow);
    }

    #[test]
    fn replace_all_colors() {
        let shadow = "20px 35px 60px -15px rgba(0,0,0,0.3),0 72px rgba(0,2,42,0.2),inset 23px 42em rgba(255,0,0,1)";
        let mut result = ShadowList::parse(shadow).unwrap();
        result.replace_all_colors("var(--en-shadow, {})");
        assert_eq!(&result.to_string(), "20px 35px 60px -15px var(--en-shadow, rgba(0,0,0,0.3)),0 72px var(--en-shadow, rgba(0,2,42,0.2)),inset 23px 42em var(--en-shadow, rgba(255,0,0,1))");
    }

    #[test]
    fn empty_shadow_should_not_panic() {
        let shadow = "";
        let None = ShadowList::parse(shadow) else {
            unreachable!("an empty shadow should not be parsed correctly");
        };
    }
}
