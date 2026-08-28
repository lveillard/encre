#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_NUMBER: StaticPlugin = Plugin::Number(Number {
    namespace: "decoration",
    prop: SingleProp("text-decoration-thickness"),
    template: Some(SingleProp("{}px")),
    ..Number::default()
});

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("text-decoration-thickness"),
    values: map! {
        "decoration-auto" => "auto",
        "decoration-from-font" => "from-font",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "decoration",
    prop: SingleProp("text-decoration-thickness"),
    hints: Some(&[ArbitraryHint::Length, ArbitraryHint::Percentage]),
    matchers: Some((
        &[
            PluginArbitraryMatcher::Length,
            PluginArbitraryMatcher::Percentage,
            PluginArbitraryMatcher::CustomMultiple(&["auto", "from-font"]),
        ],
        PluginArbitraryMatcherSeparation::None,
    )),
    ..Arbitrary::default()
});
