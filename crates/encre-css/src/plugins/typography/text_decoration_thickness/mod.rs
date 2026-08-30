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
    disambiguate: Some(ArbitraryDisambiguate {
        matchers: &[
            PluginArbitraryMatcher::Length,
            PluginArbitraryMatcher::Percentage,
            PluginArbitraryMatcher::CustomMultiple(&["auto", "from-font"]),
        ],
        matcher_separation: PluginArbitraryMatcherSeparation::None,
        hints: &[ArbitraryHint::Length, ArbitraryHint::Percentage],
    }),
    ..Arbitrary::default()
});
