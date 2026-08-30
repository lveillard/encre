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
    disambiguate: Some(ArbitraryDisambiguate {
        matched: &[
            CssType::Length,
            CssType::Percentage,
        ],
        separation: ArbitraryDisambiguateSeparation::Both,
    }),
    ..Arbitrary::default()
});
