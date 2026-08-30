#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "stroke",
    prop: SingleProp("stroke-width"),
    template: Some(SingleProp("{}px")),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "stroke",
    prop: SingleProp("stroke-width"),
    disambiguate: Some(ArbitraryDisambiguate {
        matched: &[
            CssType::Length,
            CssType::Percentage,
            CssType::LineWidth,
            CssType::Number,
        ],
        separation: ArbitraryDisambiguateSeparation::None,
    }),
    ..Arbitrary::default()
});
