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
    hints: Some(&[ArbitraryHint::Length, ArbitraryHint::Percentage]),
    matchers: Some((
        &[
            PluginArbitraryMatcher::Length,
            PluginArbitraryMatcher::Percentage,
            PluginArbitraryMatcher::LineWidth,
            PluginArbitraryMatcher::Number,
        ],
        PluginArbitraryMatcherSeparation::Comma,
    )),
    ..Arbitrary::default()
});
