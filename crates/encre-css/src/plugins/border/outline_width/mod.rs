#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "outline",
    prop: SingleProp("outline-width"),
    has_empty: Some(true),
    template: Some(SingleProp("{}px")),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "outline",
    prop: SingleProp("outline-width"),
    disambiguate: Some(ArbitraryDisambiguate {
        matched: &[CssType::Length, CssType::LineWidth],
        separation: ArbitraryDisambiguateSeparation::None,
    }),
    ..Arbitrary::default()
});
