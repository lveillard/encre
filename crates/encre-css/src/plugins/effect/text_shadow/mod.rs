#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::{plugins::PluginArbitraryMatcher::*, prelude::build_plugin::*};

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("text-shadow"),
    values: phf_map! {
        "text-shadow-2xs" => "0px 1px 0px var(--en-text-shadow-color, rgb(0 0 0 / 0.15))",
        "text-shadow-xs" => "0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.2))",
        "text-shadow-sm" => "0px 1px 0px var(--en-text-shadow-color, rgb(0 0 0 / 0.075)), 0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.075)), 0px 2px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.075))",
        "text-shadow-md" => "0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 1px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 2px 4px var(--en-text-shadow-color, rgb(0 0 0 / 0.1))",
        "text-shadow-lg" => "0px 1px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 3px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 4px 8px var(--en-text-shadow-color, rgb(0 0 0 / 0.1))",
        "text-shadow-none" => "none",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "text-shadow",
    prop: SingleProp("text-shadow"),
})
.hints(&[ArbitraryHint::Shadow])
.matcher(Shadow)
.shadow_color_replacement("var(--en-text-shadow-color, {})");
