#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Color(Color {
    namespace: "shadow",
    prop: SingleProp("--en-shadow-color"),
    ..Color::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "shadow",
    prop: SingleProp("--en-shadow-color"),
    disambiguate: Some(ArbitraryDisambiguate {
        matched: &[CssType::Color],
        separation: ArbitraryDisambiguateSeparation::None,
    }),
    ..Arbitrary::default()
});

pub(crate) const PLUGIN_INSET_1: StaticPlugin = Plugin::Color(Color {
    namespace: "inset-shadow",
    prop: SingleProp("--en-inset-shadow-color"),
    ..Color::default()
});

pub(crate) const PLUGIN_INSET_2: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "inset-shadow",
    prop: SingleProp("--en-inset-shadow-color"),
    disambiguate: Some(ArbitraryDisambiguate {
        matched: &[CssType::Color],
        separation: ArbitraryDisambiguateSeparation::None,
    }),
    ..Arbitrary::default()
});
