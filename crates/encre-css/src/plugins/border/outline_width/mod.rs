#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "outline",
    prop: SingleProp("outline-width"),
    has_empty: Some(true),
    template: Some("{}px"),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "outline",
    prop: SingleProp("outline-width"),
    hints: Some(&[ArbitraryHint::Length]),
    matchers: Some((
        &[
            PluginArbitraryMatcher::Length,
            PluginArbitraryMatcher::LineWidth,
        ],
        PluginArbitraryMatcherSeparation::None,
    )),
    ..Arbitrary::default()
});
