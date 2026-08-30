#![doc = include_str!("README.md")]
#![doc(alias = "outline")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Color(Color {
    namespace: "outline",
    prop: SingleProp("outline-color"),
    ..Color::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "outline",
    prop: SingleProp("outline-color"),
    disambiguate: Some(ArbitraryDisambiguate {
        matchers: &[PluginArbitraryMatcher::Color],
        matcher_separation: PluginArbitraryMatcherSeparation::None,
        hints: &[ArbitraryHint::Color],
    }),
    ..Arbitrary::default()
});
