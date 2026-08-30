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
        matched: &[CssType::Length, CssType::Percentage],
        separation: ArbitraryDisambiguateSeparation::None,
    }),
    ..Arbitrary::default()
});
