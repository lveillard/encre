#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("background-position"),
    values: map! {
        "bg-bottom" => "bottom",
        "bg-center" => "center",
        "bg-left" => "left",
        "bg-left-bottom" => "left bottom",
        "bg-left-top" => "left top",
        "bg-right" => "right",
        "bg-right-bottom" => "right bottom",
        "bg-right-top" => "right top",
        "bg-top" => "top",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "bg",
    prop: SingleProp("background-position"),
    disambiguate: Some(ArbitraryDisambiguate {
        matchers: &[PluginArbitraryMatcher::Position],
        matcher_separation: PluginArbitraryMatcherSeparation::Both,
        hints: &[ArbitraryHint::Position],
    }),
    ..Arbitrary::default()
});
