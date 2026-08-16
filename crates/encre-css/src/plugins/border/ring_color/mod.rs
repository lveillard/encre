#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Color(Color {
    namespace: "ring",
    prop: SingleProp("--en-ring-color"),
    ..Color::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "ring",
    prop: SingleProp("--en-ring-color"),
    hints: Some(&[ArbitraryHint::Color]),
    matchers: Some((
        &[PluginArbitraryMatcher::Color],
        PluginArbitraryMatcherSeparation::None,
    )),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_INSET_1: StaticPlugin = Plugin::Color(Color {
    namespace: "inset-ring",
    prop: SingleProp("--en-inset-ring-color"),
    ..Color::default()
});

pub(crate) const PLUGIN_INSET_2: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "inset-ring",
    prop: SingleProp("--en-inset-ring-color"),
    hints: Some(&[ArbitraryHint::Color]),
    matchers: Some((
        &[PluginArbitraryMatcher::Color],
        PluginArbitraryMatcherSeparation::None,
    )),
    ..Arbitrary::default()
});
