#![doc = include_str!("README.md")]
#![doc(alias("background", "bg", "gradient"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

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

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-image"),
    values: phf_map! {
        "bg-none" => "none",
        "bg-gradient-to-t" => "linear-gradient(to top in oklab, var(--en-gradient-stops))",
        "bg-gradient-to-tr" => "linear-gradient(to top right in oklab, var(--en-gradient-stops))",
        "bg-gradient-to-r" => "linear-gradient(to right in oklab, var(--en-gradient-stops))",
        "bg-gradient-to-br" => "linear-gradient(to bottom right in oklab, var(--en-gradient-stops))",
        "bg-gradient-to-b" => "linear-gradient(to bottom in oklab, var(--en-gradient-stops))",
        "bg-gradient-to-bl" => "linear-gradient(to bottom left in oklab, var(--en-gradient-stops))",
        "bg-gradient-to-l" => "linear-gradient(to left in oklab, var(--en-gradient-stops))",
        "bg-gradient-to-tl" => "linear-gradient(to top left in oklab, var(--en-gradient-stops))",
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "bg",
    prop: SingleProp("background-image"),
})
.hints(&[ArbitraryHint::Url, ArbitraryHint::Image])
.matchers(&[Image], PluginArbitraryMatcherModifier::None);

pub(crate) const PLUGIN_LINEAR_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-image"),
    values: phf_map! {
        "bg-linear-none" => "none",
        "bg-linear-to-t" => "linear-gradient(to top in {/}, var(--en-gradient-stops))",
        "bg-linear-to-tr" => "linear-gradient(to top right in {/}, var(--en-gradient-stops))",
        "bg-linear-to-r" => "linear-gradient(to right in {/}, var(--en-gradient-stops))",
        "bg-linear-to-br" => "linear-gradient(to bottom right in {/}, var(--en-gradient-stops))",
        "bg-linear-to-b" => "linear-gradient(to bottom in {/}, var(--en-gradient-stops))",
        "bg-linear-to-bl" => "linear-gradient(to bottom left in {/}, var(--en-gradient-stops))",
        "bg-linear-to-l" => "linear-gradient(to left in {/}, var(--en-gradient-stops))",
        "bg-linear-to-tl" => "linear-gradient(to top left in {/}, var(--en-gradient-stops))",
    },
})
.extra_slash(INTERPOLATION_MODES_MAP, "oklab");

pub(crate) const PLUGIN_LINEAR_2: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "bg-linear",
    prop: SingleProp("background-image"),
})
.has_negative()
.template("linear-gradient({}deg in {/}, var(--en-gradient-stops))")
.extra_slash(INTERPOLATION_MODES_MAP, "oklab");

pub(crate) const PLUGIN_LINEAR_3: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "bg-linear",
    prop: SingleProp("background-image"),
})
.template("linear-gradient({})");

pub(crate) const PLUGIN_RADIAL_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-image"),
    values: phf_map! {
        "bg-radial" => "radial-gradient(in {/}, var(--en-gradient-stops))",
    },
})
.extra_slash(INTERPOLATION_MODES_MAP, "oklab");

pub(crate) const PLUGIN_RADIAL_2: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "bg-radial",
    prop: SingleProp("background-image"),
})
.template("radial-gradient({})");

pub(crate) const PLUGIN_CONIC_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-image"),
    values: phf_map! {
        "bg-conic" => "conic-gradient(in {/}, var(--en-gradient-stops))",
    },
})
.extra_slash(INTERPOLATION_MODES_MAP, "oklab");

pub(crate) const PLUGIN_CONIC_2: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "bg-conic",
    prop: SingleProp("background-image"),
})
.has_negative()
.template("conic-gradient(from {}deg in {/}, var(--en-gradient-stops))")
.extra_slash(INTERPOLATION_MODES_MAP, "oklab");

pub(crate) const PLUGIN_CONIC_3: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "bg-conic",
    prop: SingleProp("background-image"),
})
.template("conic-gradient({})");
