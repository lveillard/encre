#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Color(Color {
    namespace: "text",
    prop: SingleProp("color"),
    ..Color::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "text",
    prop: SingleProp("color"),
    hints: Some(&[ArbitraryHint::Color]),
    matchers: Some((
        &[PluginArbitraryMatcher::Color],
        PluginArbitraryMatcherSeparation::None,
    )),
    ..Arbitrary::default()
});
