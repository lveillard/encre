#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Color(Color {
    namespace: "stroke",
    prop: SingleProp("stroke"),
    ..Color::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "stroke",
    prop: SingleProp("stroke"),
    disambiguate: Some(ArbitraryDisambiguate {
        matchers: &[PluginArbitraryMatcher::Color],
        matcher_separation: PluginArbitraryMatcherSeparation::None,
        hints: &[ArbitraryHint::Color],
    }),
    ..Arbitrary::default()
});
