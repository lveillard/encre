#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Color(Color {
    namespace: "ring-offset",
    prop: SingleProp("--en-ring-offset-color"),
    extra_rule_css: Some(&[super::ring_offset_width::RING_OFFSET_SHADOW]),
    ..Color::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "ring-offset",
    prop: SingleProp("--en-ring-offset-color"),
    disambiguate: Some(ArbitraryDisambiguate {
        matchers: &[PluginArbitraryMatcher::Color],
        matcher_separation: PluginArbitraryMatcherSeparation::None,
        hints: &[ArbitraryHint::Color],
    }),
    extra_rule_css: Some(&[super::ring_offset_width::RING_OFFSET_SHADOW]),
    ..Arbitrary::default()
});
