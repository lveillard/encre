#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

const BOX_SHADOW: &str = "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);";
const INSET_BOX_SHADOW: &str = "box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);";

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "ring",
    prop: SingleProp("--en-ring-shadow"),
    has_empty: Some(true),
    extra_lines: Some(&[BOX_SHADOW]),
    template: Some("0 0 0 calc({}px + var(--en-ring-offset-width)) var(--en-ring-color)"),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "ring",
    prop: SingleProp("--en-ring-shadow"),
    extra_lines: Some(&[BOX_SHADOW]),
    hints: Some(&[ArbitraryHint::Length]),
    matchers: Some((
        &[PluginArbitraryMatcher::Length],
        PluginArbitraryMatcherSeparation::None,
    )),
    template: Some(
        "var(--en-ring-inset) 0 0 0 calc({} + var(--en-ring-offset-width)) var(--en-ring-color)",
    ),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_INSET_1: StaticPlugin = Plugin::Number(Number {
    namespace: "inset-ring",
    prop: SingleProp("--en-inset-ring-shadow"),
    has_empty: Some(true),
    extra_lines: Some(&[INSET_BOX_SHADOW]),
    template: Some("inset 0 0 0 calc({}px + var(--en-ring-offset-width)) var(--en-ring-color)"),
    ..Number::default()
});

pub(crate) const PLUGIN_INSET_2: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "inset-ring",
    prop: SingleProp("--en-inset-ring-shadow"),
    extra_lines: Some(&[INSET_BOX_SHADOW]),
    hints: Some(&[ArbitraryHint::Length]),
    matchers: Some((
        &[PluginArbitraryMatcher::Length],
        PluginArbitraryMatcherSeparation::None,
    )),
    template: Some("inset 0 0 0 calc({value} + var(--en-ring-offset-width)) var(--en-ring-color)"),
    ..Arbitrary::default()
});
