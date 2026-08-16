#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Color(Color {
    namespace: "bg",
    prop: SingleProp("background-color"),
    ..Color::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "bg",
    prop: SingleProp("background-color"),
    hints: Some(&[ArbitraryHint::Color]),
    matchers: Some((&[PluginArbitraryMatcher::Color], PluginArbitraryMatcherSeparation::None)),
    ..Arbitrary::default()
});
