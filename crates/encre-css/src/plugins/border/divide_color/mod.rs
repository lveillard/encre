#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Color(Color {
    namespace: "divide",
    prop: SingleProp("border-color"),
    extra_class: Some(" > :not([hidden]) ~ :not([hidden])"),
    ..Color::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "divide",
    prop: SingleProp("border-color"),
    disambiguate: Some(ArbitraryDisambiguate {
        matchers: &[PluginArbitraryMatcher::Color],
        matcher_separation: PluginArbitraryMatcherSeparation::None,
        hints: &[ArbitraryHint::Color],
    }),
    extra_class: Some(" > :not([hidden]) ~ :not([hidden])"),
    ..Arbitrary::default()
});
