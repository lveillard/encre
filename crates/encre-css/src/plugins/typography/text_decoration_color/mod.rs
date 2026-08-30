#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Color(Color {
    namespace: "decoration",
    prop: MultipleProps(&["-webkit-text-decoration-color", "text-decoration-color"]),
    ..Color::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "decoration",
    prop: MultipleProps(&["-webkit-text-decoration-color", "text-decoration-color"]),
    disambiguate: Some(ArbitraryDisambiguate {
        matchers: &[PluginArbitraryMatcher::Color],
        matcher_separation: PluginArbitraryMatcherSeparation::None,
        hints: &[ArbitraryHint::Color],
    }),
    ..Arbitrary::default()
});
