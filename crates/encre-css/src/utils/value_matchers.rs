//! Define some utility functions used to detect the [CSS type](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types) of a CSS value.
use super::shadow::ShadowList;

const LENGTH_UNITS: [&str; 16] = [
    "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "vw", "vh", "vmin",
    "vmax",
];
const LINE_WIDTHS: [&str; 3] = ["thin", "medium", "thick"];
const LINE_STYLES: [&str; 10] = [
    "solid", "dashed", "dotted", "double", "groove", "ridge", "inset", "outset", "hidden", "none",
];
const ANGLES: [&str; 4] = ["deg", "grad", "rad", "turn"];
const GRADIENT_TYPES: [&str; 5] = [
    "linear-gradient",
    "radial-gradient",
    "repeating-linear-gradient",
    "repeating-radial-gradient",
    "conic-gradient",
];
const VALID_POSITIONS: [&str; 5] = ["center", "top", "right", "bottom", "left"];
const ABSOLUTE_SIZES: [&str; 8] = [
    "xx-small",
    "x-small",
    "small",
    "medium",
    "large",
    "x-large",
    "xx-large",
    "xxx-large",
];
const RELATIVE_SIZES: [&str; 2] = ["larger", "smaller"];
const NAMED_COLORS: [&str; 150] = [
    "transparent",
    "currentColor",
    "antiquewhite",
    "aliceblue",
    "aqua",
    "aquamarine",
    "azure",
    "beige",
    "bisque",
    "black",
    "blanchedalmond",
    "blue",
    "blueviolet",
    "brown",
    "burlywood",
    "cadetblue",
    "chartreuse",
    "chocolate",
    "coral",
    "cornflowerblue",
    "cornsilk",
    "crimson",
    "cyan",
    "darkblue",
    "darkcyan",
    "darkgoldenrod",
    "darkgray",
    "darkgreen",
    "darkgrey",
    "darkkhaki",
    "darkmagenta",
    "darkolivegreen",
    "darkorange",
    "darkorchid",
    "darkred",
    "darksalmon",
    "darkseagreen",
    "darkslateblue",
    "darkslategray",
    "darkslategrey",
    "darkturquoise",
    "darkviolet",
    "deeppink",
    "deepskyblue",
    "dimgray",
    "dimgrey",
    "dodgerblue",
    "firebrick",
    "floralwhite",
    "forestgreen",
    "fuchsia",
    "gainsboro",
    "ghostwhite",
    "gold",
    "goldenrod",
    "gray",
    "green",
    "greenyellow",
    "grey",
    "honeydew",
    "hotpink",
    "indianred",
    "indigo",
    "ivory",
    "khaki",
    "lavender",
    "lavenderblush",
    "lawngreen",
    "lemonchiffon",
    "lightblue",
    "lightcoral",
    "lightcyan",
    "lightgoldenrodyellow",
    "lightgray",
    "lightgreen",
    "lightgrey",
    "lightpink",
    "lightsalmon",
    "lightseagreen",
    "lightskyblue",
    "lightslategray",
    "lightslategrey",
    "lightsteelblue",
    "lightyellow",
    "lime",
    "limegreen",
    "linen",
    "magenta",
    "maroon",
    "mediumaquamarine",
    "mediumblue",
    "mediumorchid",
    "mediumpurple",
    "mediumseagreen",
    "mediumslateblue",
    "mediumspringgreen",
    "mediumturquoise",
    "mediumvioletred",
    "midnightblue",
    "mintcream",
    "mistyrose",
    "moccasin",
    "navajowhite",
    "navy",
    "oldlace",
    "olive",
    "olivedrab",
    "orange",
    "orangered",
    "orchid",
    "palegoldenrod",
    "palegreen",
    "paleturquoise",
    "palevioletred",
    "papayawhip",
    "peachpuff",
    "peru",
    "pink",
    "plum",
    "powderblue",
    "purple",
    "rebeccapurple",
    "red",
    "rosybrown",
    "royalblue",
    "saddlebrown",
    "salmon",
    "sandybrown",
    "seagreen",
    "seashell",
    "sienna",
    "silver",
    "skyblue",
    "slateblue",
    "slategray",
    "slategrey",
    "snow",
    "springgreen",
    "steelblue",
    "tan",
    "teal",
    "thistle",
    "tomato",
    "turquoise",
    "violet",
    "wheat",
    "white",
    "whitesmoke",
    "yellow",
    "yellowgreen",
];
const GENERIC_FONT_FAMILIES: &[&str] = &[
    "serif",
    "sans-serif",
    "monospace",
    "cursive",
    "fantasy",
    "system-ui",
    "ui-serif",
    "ui-sans-serif",
    "ui-monospace",
    "ui-rounded",
    "math",
    "fangsong",
];

fn is_matching_base(value: &str) -> bool {
    is_matching_var(value)
        || [
            "inherit",
            "initial",
            "revert",
            "revert-layer",
            "unset",
            "fill",
            "max-content",
            "min-content",
            "fit-content",
        ]
        .contains(&value)
}

/// Match all CSS types.
pub fn is_matching_all(_value: &str) -> bool {
    true
}

/// Returns whether the CSS value is an [`url()`](https://developer.mozilla.org/en-US/docs/Web/CSS/url).
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_url;
/// assert!(is_matching_url("url('/hello/world.png')"));
/// ```
pub fn is_matching_url(value: &str) -> bool {
    value.starts_with("url(")
}

/// Returns whether the CSS value is a [`var()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Using_CSS_custom_properties).
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_var;
/// assert!(is_matching_var("var(--bg-blue)"));
/// ```
pub fn is_matching_var(value: &str) -> bool {
    value.starts_with("var(")
}

/// Returns whether the CSS value is a [`shadow`](https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow#values).
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_shadow;
/// assert!(is_matching_shadow("1px 2rem 10px 10px rgb(12,12,12)"));
/// ```
pub fn is_matching_shadow(value: &str) -> bool {
    ShadowList::parse(value).is_some()
}

/// Returns whether the CSS value is an [`absolute size`](https://developer.mozilla.org/en-US/docs/Web/CSS/font-size#values).
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_absolute_size;
/// assert!(is_matching_absolute_size("xx-small"));
/// ```
pub fn is_matching_absolute_size(value: &str) -> bool {
    ABSOLUTE_SIZES.contains(&value)
}

/// Returns whether the CSS value is a [`relative size`](https://developer.mozilla.org/en-US/docs/Web/CSS/font-size#values).
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_relative_size;
/// assert!(is_matching_relative_size("larger"));
/// ```
pub fn is_matching_relative_size(value: &str) -> bool {
    RELATIVE_SIZES.contains(&value)
}

/// Returns whether the CSS value is a [`line width`](https://developer.mozilla.org/en-US/docs/Web/CSS/border-width#values).
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_line_width;
/// assert!(is_matching_line_width("thin"));
/// ```
pub fn is_matching_line_width(value: &str) -> bool {
    LINE_WIDTHS.contains(&value)
}

/// Returns whether the CSS value is a [`line style`](https://developer.mozilla.org/en-US/docs/Web/CSS/border-style#values).
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_line_style;
/// assert!(is_matching_line_style("solid"));
/// ```
pub fn is_matching_line_style(value: &str) -> bool {
    LINE_STYLES.contains(&value)
}

/// Returns whether the CSS value is a computational CSS function like:
///
/// - [`min()`](https://developer.mozilla.org/en-US/docs/Web/CSS/min)
/// - [`max()`](https://developer.mozilla.org/en-US/docs/Web/CSS/max)
/// - [`clamp()`](https://developer.mozilla.org/en-US/docs/Web/CSS/clamp)
/// - [`calc()`](https://developer.mozilla.org/en-US/docs/Web/CSS/calc)
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_computational_css_function;
/// assert!(is_matching_computational_css_function("min(12px,10%)"));
/// ```
pub fn is_matching_computational_css_function(value: &str) -> bool {
    value.starts_with("min(")
        || value.starts_with("max(")
        || value.starts_with("clamp(")
        || value.starts_with("calc(")
}

/// Returns whether the CSS value has the [`<color>`](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value) type.
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_color;
/// assert!(is_matching_color("blue"));
/// assert!(is_matching_color("#333"));
/// ```
pub fn is_matching_color(value: &str) -> bool {
    (value.starts_with('#') && (value.len() == 4 || value.len() == 7))
        || ["rgb(", "rgba(", "hsl(", "hsla(", "hwb(", "lch(", "lab("]
            .iter()
            .any(|e| value.starts_with(e))
        || NAMED_COLORS.iter().any(|c| &value == c)
        || is_matching_base(value)
}

/// Returns whether the CSS value has the [`<length>`](https://developer.mozilla.org/en-US/docs/Web/CSS/length) type.
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_length;
/// assert!(is_matching_length("300px"));
/// ```
pub fn is_matching_length(value: &str) -> bool {
    value == "0"
        || LENGTH_UNITS.iter().any(|u| value.ends_with(u))
        || is_matching_computational_css_function(value)
        || is_matching_base(value)
}

/// Returns whether the CSS value has the [`<number>`](https://developer.mozilla.org/en-US/docs/Web/CSS/number) type.
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_number;
/// assert!(is_matching_number("42.12"));
/// ```
pub fn is_matching_number(value: &str) -> bool {
    value.parse::<f32>().is_ok()
        || is_matching_computational_css_function(value)
        || is_matching_base(value)
}

/// Returns whether the CSS value has the [`<percentage>`](https://developer.mozilla.org/en-US/docs/Web/CSS/percentage) type.
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_percentage;
/// assert!(is_matching_percentage("10%"));
/// ```
pub fn is_matching_percentage(value: &str) -> bool {
    value.ends_with('%') || is_matching_computational_css_function(value) || is_matching_base(value)
}

/// Returns whether the CSS value has the [`<time>`](https://developer.mozilla.org/en-US/docs/Web/CSS/time) type.
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_time;
/// assert!(is_matching_time("0.5s"));
/// assert!(is_matching_time("10ms"));
/// ```
pub fn is_matching_time(value: &str) -> bool {
    value.ends_with('s')
        || value.ends_with("ms")
        || is_matching_computational_css_function(value)
        || is_matching_base(value)
}

/// Returns whether the CSS value has the [`<gradient>`](https://developer.mozilla.org/en-US/docs/Web/CSS/gradient) type.
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_gradient;
/// assert!(is_matching_gradient("linear-gradient(45deg, blue, red);"));
/// ```
pub fn is_matching_gradient(value: &str) -> bool {
    GRADIENT_TYPES.iter().any(|t| value.starts_with(t)) || is_matching_base(value)
}

/// Returns whether the CSS value has the [`<position>`](https://developer.mozilla.org/en-US/docs/Web/CSS/position_value) type.
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_position;
/// assert!(is_matching_position("right"));
/// assert!(is_matching_position("12px"));
/// assert!(is_matching_position("42%"));
/// ```
pub fn is_matching_position(value: &str) -> bool {
    VALID_POSITIONS.contains(&value)
        || is_matching_length(value)
        || is_matching_percentage(value)
        || is_matching_base(value)
}

/// Returns whether the CSS value has the [`<angle>`](https://developer.mozilla.org/en-US/docs/Web/CSS/angle) type.
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_angle;
/// assert!(is_matching_angle("0.2turn"));
/// ```
pub fn is_matching_angle(value: &str) -> bool {
    ANGLES.iter().any(|a| value.ends_with(a))
        || is_matching_computational_css_function(value)
        || is_matching_base(value)
}

/// Returns whether the CSS value has the [`<image>`](https://developer.mozilla.org/en-US/docs/Web/CSS/image) type.
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_image;
/// assert!(is_matching_image("linear-gradient(to_right,red,orange,yellow,green,blue,indigo,violet)"));
/// ```
pub fn is_matching_image(value: &str) -> bool {
    is_matching_url(value)
        || is_matching_gradient(value)
        || ["element(", "image(", "cross-fade(", "image-set("]
            .iter()
            .any(|e| value.starts_with(e))
        || is_matching_base(value)
}

/// Returns whether the CSS value is a font family name.
///
/// A font family name does not have a dedicated CSS type but it's a mix of
/// `<generic-font-family>`, a string, or a custom identifier, and can be used in properties like
/// `font-family`.
///
/// # Example
///
/// ```
/// use encre_css::utils::value_matchers::is_matching_font_family_name;
/// assert!(is_matching_font_family_name("sans-serif"));
/// assert!(is_matching_font_family_name("'Open Sans'"));
/// assert!(!is_matching_font_family_name("12px"));
/// ```
pub fn is_matching_font_family_name(value: &str) -> bool {
    GENERIC_FONT_FAMILIES.contains(&value)
        || value.chars().next().is_some_and(|ch| !ch.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_matching_color_test() {
        assert!(is_matching_color("blue"));
        assert!(is_matching_color("#333"));
        assert!(is_matching_color("#121212"));
        assert!(is_matching_color("rgb(12.12,12,12)"));
        assert!(is_matching_color("rgb(12 12 12)"));
        assert!(is_matching_color("rgb(12 12 12/0.1)"));
        assert!(is_matching_color("rgb(12 12 12 / 0.1)"));
        assert!(is_matching_color("rgb(var(--blue),12,12)"));
        assert!(is_matching_color("rgb(12 12 12 / var(--opacity))"));
        assert!(is_matching_color("rgba(12,12,12,0.12)"));
        assert!(is_matching_color("hsl(360,100%,50%)"));
        assert!(is_matching_color("hsl(3.14rad,100%,50%)"));
        assert!(is_matching_color("hsl(3.14rad 100% 50%/0.42)"));
        assert!(is_matching_color("hsl(var(--hue) 12% 42%/var(--opacity))"));
        assert!(is_matching_color("hsla(360,100%,50%,0.12)"));
    }

    #[test]
    fn is_matching_length_test() {
        assert!(is_matching_length("300px"));
        assert!(!is_matching_length("50%"));
        assert!(is_matching_length("30vw"));
        assert!(is_matching_length("min(10%,10px)"));
        assert!(is_matching_length("0"));
    }

    #[test]
    fn is_matching_shadow_with_functions_test() {
        assert!(is_matching_shadow("10px 10px min(1px,2px) 10px rgb(1,1,1)"));
        assert!(is_matching_shadow(
            "inset 0 -3em 3em rgba(0,0,0,0.1),0 0 0 2px rgb(255,255,255),0.3em 0.3em 1em rgba(0,0,0,0.3)"
        ));
        assert!(is_matching_shadow(
            "var(--a, 0 0 1px rgb(0, 0, 0)), 0 0 1px rgb(0, 0, 0)"
        ));
    }
}
