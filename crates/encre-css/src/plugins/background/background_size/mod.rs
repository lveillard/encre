#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-size"),
    values: phf_map! {
        "bg-auto" => "auto",
        "bg-cover" => "cover",
        "bg-contain" => "contain",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "bg",
    prop: SingleProp("background-size"),
})
.hints(&[ArbitraryHint::Length, ArbitraryHint::Percentage])
.matcher(CommaSeparated(&OrMultiple(&[
    &SpaceSeparated(&OrMultiple(&[&Length, &Percentage, &Custom("auto")])),
    &Custom("cover"),
    &Custom("contain"),
])));
