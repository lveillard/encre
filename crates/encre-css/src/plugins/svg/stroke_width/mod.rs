#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "stroke",
    prop: SingleProp("stroke-width"),
    template: Some(SingleProp("{}px")),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "stroke",
    prop: SingleProp("stroke-width"),
    disambiguate: Some(ArbitraryDisambiguate {
        matchers: &[
            PluginArbitraryMatcher::Length,
            PluginArbitraryMatcher::Percentage,
            PluginArbitraryMatcher::LineWidth,
            PluginArbitraryMatcher::Number,
        ],
        matcher_separation: PluginArbitraryMatcherSeparation::Comma,
        hints: &[ArbitraryHint::Length, ArbitraryHint::Percentage],
    }),
    ..Arbitrary::default()
});
