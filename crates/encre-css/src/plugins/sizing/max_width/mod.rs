#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::Spacing {
    prop: SingleProp("max-width"),
    has_auto: false,
    has_full: false,
};

pub(crate) const PLUGIN_2: Plugin = Plugin::ListValues {
    prop: SingleProp("max-width"),
    values: phf_map! {
        "none" => "none",
        "xs" => "20rem",
        "sm" => "24rem",
        "md" => "28rem",
        "lg" => "32rem",
        "xl" => "36rem",
        "2xl" => "42rem",
        "3xl" => "48rem",
        "4xl" => "56rem",
        "5xl" => "64rem",
        "6xl" => "72rem",
        "7xl" => "80rem",
        "full" => "100%",
        "min" => "min-content",
        "max" => "max-content",
        "fit" => "fit-content",
        "prose" => "65ch",
        "screen" => "100vw",
        "screen-sm" => "640px",
        "screen-md" => "768px",
        "screen-lg" => "1024px",
        "screen-xl" => "1280px",
        "screen-2xl" => "1536px",
    },
};

pub(crate) const PLUGIN_3: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("max-width"),
    hints: &[PluginArbitraryHint::Length, PluginArbitraryHint::Percentage],
    matcher: Or(&Length, &Percentage),
};
