#![doc = include_str!("README.md")]
#![doc(alias("background", "bg", "gradient"))]
use crate::prelude::build_plugin::*;

const INTERPOLATION_MODES_MAP: phf::Map<&'static str, &'static str> = map! {
    "longer" => "oklch longer hue",
    "shorter" => "oklch shorter hue",
    "increasing" => "oklch increasing hue",
    "decreasing" => "oklch decreasing hue",
    "srgb" => "srgb",
    "hsl" => "hsl",
    "oklab" => "oklab",
    "oklch" => "oklch",
};

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("background-image"),
    values: map! {
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
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "bg",
    prop: SingleProp("background-image"),
    hints: Some(&[ArbitraryHint::Url, ArbitraryHint::Image]),
    matchers: Some((
        &[PluginArbitraryMatcher::Image],
        PluginArbitraryMatcherSeparation::None,
    )),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_LINEAR_1: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("background-image"),
    values: map! {
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
    extra_slash: Some(ExtraSlash {
        values: INTERPOLATION_MODES_MAP,
        default: "oklab",
    }),
    ..ListValues::default()
});

pub(crate) const PLUGIN_LINEAR_2: StaticPlugin = Plugin::Number(Number {
    namespace: "bg-linear",
    prop: SingleProp("background-image"),
    has_negative: Some(true),
    template: Some(SingleProp("linear-gradient({}deg in {/}, var(--en-gradient-stops))")),
    extra_slash: Some(ExtraSlash {
        values: INTERPOLATION_MODES_MAP,
        default: "oklab",
    }),
    ..Number::default()
});

pub(crate) const PLUGIN_LINEAR_3: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "bg-linear",
    prop: SingleProp("background-image"),
    template: Some(SingleProp("linear-gradient({})")),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_RADIAL_1: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("background-image"),
    values: map! {
        "bg-radial" => "radial-gradient(in {/}, var(--en-gradient-stops))",
    },
    extra_slash: Some(ExtraSlash {
        values: INTERPOLATION_MODES_MAP,
        default: "oklab",
    }),
    ..ListValues::default()
});

pub(crate) const PLUGIN_RADIAL_2: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "bg-radial",
    prop: SingleProp("background-image"),
    template: Some(SingleProp("radial-gradient({})")),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_CONIC_1: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("background-image"),
    values: map! {
        "bg-conic" => "conic-gradient(in {/}, var(--en-gradient-stops))",
    },
    extra_slash: Some(ExtraSlash {
        values: INTERPOLATION_MODES_MAP,
        default: "oklab",
    }),
    ..ListValues::default()
});

pub(crate) const PLUGIN_CONIC_2: StaticPlugin = Plugin::Number(Number {
    namespace: "bg-conic",
    prop: SingleProp("background-image"),
    has_negative: Some(true),
    template: Some(SingleProp("conic-gradient(from {}deg in {/}, var(--en-gradient-stops))")),
    extra_slash: Some(ExtraSlash {
        values: INTERPOLATION_MODES_MAP,
        default: "oklab",
    }),
    ..Number::default()
});

pub(crate) const PLUGIN_CONIC_3: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "bg-conic",
    prop: SingleProp("background-image"),
    template: Some(SingleProp("conic-gradient({})")),
    ..Arbitrary::default()
});
