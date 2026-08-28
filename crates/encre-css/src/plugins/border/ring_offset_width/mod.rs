#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const RING_OFFSET_SHADOW: &str = "--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);";

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "ring-offset",
    prop: SingleProp("--en-ring-offset-width"),
    extra_rule_css: Some(&[RING_OFFSET_SHADOW]),
    template: Some(SingleProp("{}px")),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "ring-offset",
    prop: SingleProp("--en-ring-offset-width"),
    hints: Some(&[ArbitraryHint::Length]),
    matchers: Some((
        &[PluginArbitraryMatcher::Length],
        PluginArbitraryMatcherSeparation::None,
    )),
    extra_rule_css: Some(&[RING_OFFSET_SHADOW]),
    ..Arbitrary::default()
});
