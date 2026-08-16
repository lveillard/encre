#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("background-size"),
    values: map! {
        "bg-auto" => "auto",
        "bg-cover" => "cover",
        "bg-contain" => "contain",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "bg",
    prop: SingleProp("background-size"),
    hints: Some(&[ArbitraryHint::Length, ArbitraryHint::Percentage]),
    matchers: Some((
        &[
            PluginArbitraryMatcher::Length,
            PluginArbitraryMatcher::Percentage,
            PluginArbitraryMatcher::Custom("auto"),
            PluginArbitraryMatcher::Custom("cover"),
            PluginArbitraryMatcher::Custom("contain"),
        ],
        PluginArbitraryMatcherSeparation::Both,
    )),
    ..Arbitrary::default()
});
