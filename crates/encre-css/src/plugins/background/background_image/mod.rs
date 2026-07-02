#![doc = include_str!("README.md")]
#![doc(alias("background", "bg", "gradient"))]
use crate::prelude::build_plugin::*;

const INTERPOLATION_MODES_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "longer" => "oklch longer hue",
    "shorter" => "oklch shorter hue",
    "increasing" => "oklch increasing hue",
    "decreasing" => "oklch decreasing hue",
    "srgb" => "srgb",
    "hsl" => "hsl",
    "oklab" => "oklab",
    "oklch" => "oklch",
};

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-image"),
    values: phf_map! {
        "none" => "none",
        "gradient-to-t" => "linear-gradient(to top in oklab, var(--en-gradient-stops))",
        "gradient-to-tr" => "linear-gradient(to top right in oklab, var(--en-gradient-stops))",
        "gradient-to-r" => "linear-gradient(to right in oklab, var(--en-gradient-stops))",
        "gradient-to-br" => "linear-gradient(to bottom right in oklab, var(--en-gradient-stops))",
        "gradient-to-b" => "linear-gradient(to bottom in oklab, var(--en-gradient-stops))",
        "gradient-to-bl" => "linear-gradient(to bottom left in oklab, var(--en-gradient-stops))",
        "gradient-to-l" => "linear-gradient(to left in oklab, var(--en-gradient-stops))",
        "gradient-to-tl" => "linear-gradient(to top left in oklab, var(--en-gradient-stops))",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("background-image"),
    hints: &[PluginArbitraryHint::Url, PluginArbitraryHint::Image],
    matcher: Image,
});

pub(crate) const PLUGIN_LINEAR_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-image"),
    values: phf_map! {
        "none" => "none",
        "to-t" => "linear-gradient(to top in {/}, var(--en-gradient-stops))",
        "to-tr" => "linear-gradient(to top right in {/}, var(--en-gradient-stops))",
        "to-r" => "linear-gradient(to right in {/}, var(--en-gradient-stops))",
        "to-br" => "linear-gradient(to bottom right in {/}, var(--en-gradient-stops))",
        "to-b" => "linear-gradient(to bottom in {/}, var(--en-gradient-stops))",
        "to-bl" => "linear-gradient(to bottom left in {/}, var(--en-gradient-stops))",
        "to-l" => "linear-gradient(to left in {/}, var(--en-gradient-stops))",
        "to-tl" => "linear-gradient(to top left in {/}, var(--en-gradient-stops))",
    },
})
.extra_slash(INTERPOLATION_MODES_MAP, "oklab");

pub(crate) const PLUGIN_LINEAR_2: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("background-image"),
    has_empty: false,
    has_negative: true,
    divide_by: 1.0,
})
.template("linear-gradient({}deg in {/}, var(--en-gradient-stops))")
.extra_slash(INTERPOLATION_MODES_MAP, "oklab");

pub(crate) const PLUGIN_LINEAR_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("background-image"),
    hints: &[],
    matcher: All,
})
.template("linear-gradient({})");

pub(crate) const PLUGIN_RADIAL_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-image"),
    values: phf_map! {
        "" => "radial-gradient(in {/}, var(--en-gradient-stops))",
    },
})
.extra_slash(INTERPOLATION_MODES_MAP, "oklab");

pub(crate) const PLUGIN_RADIAL_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("background-image"),
    hints: &[],
    matcher: All,
})
.template("radial-gradient({})");

pub(crate) const PLUGIN_CONIC_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-image"),
    values: phf_map! {
        "" => "conic-gradient(in {/}, var(--en-gradient-stops))",
    },
})
.extra_slash(INTERPOLATION_MODES_MAP, "oklab");

pub(crate) const PLUGIN_CONIC_2: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("background-image"),
    has_empty: false,
    has_negative: true,
    divide_by: 1.0,
})
.template("conic-gradient(from {}deg in {/}, var(--en-gradient-stops))")
.extra_slash(INTERPOLATION_MODES_MAP, "oklab");

pub(crate) const PLUGIN_CONIC_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("background-image"),
    hints: &[],
    matcher: All,
})
.template("conic-gradient({})");
