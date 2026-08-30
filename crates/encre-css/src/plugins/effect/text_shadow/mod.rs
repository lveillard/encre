#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("text-shadow"),
    values: map! {
        "text-shadow-2xs" => "0px 1px 0px var(--en-text-shadow-color, rgb(0 0 0 / 0.15))",
        "text-shadow-xs" => "0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.2))",
        "text-shadow-sm" => "0px 1px 0px var(--en-text-shadow-color, rgb(0 0 0 / 0.075)), 0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.075)), 0px 2px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.075))",
        "text-shadow-md" => "0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 1px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 2px 4px var(--en-text-shadow-color, rgb(0 0 0 / 0.1))",
        "text-shadow-lg" => "0px 1px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 3px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 4px 8px var(--en-text-shadow-color, rgb(0 0 0 / 0.1))",
        "text-shadow-none" => "none",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "text-shadow",
    prop: SingleProp("text-shadow"),
    disambiguate: Some(ArbitraryDisambiguate {
        matchers: &[PluginArbitraryMatcher::Shadow],
        matcher_separation: PluginArbitraryMatcherSeparation::None,
        hints: &[ArbitraryHint::Shadow],
    }),
    shadow_color_replacement: Some("var(--en-text-shadow-color, {})"),
    ..Arbitrary::default()
});
