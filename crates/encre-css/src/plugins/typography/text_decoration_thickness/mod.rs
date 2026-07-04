#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::AnyNumber {
    prefix: "decoration",
    prop: SingleProp("text-decoration-thickness"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
})
.template("{}px");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("text-decoration-thickness"),
    values: phf_map! {
        "decoration-auto" => "auto",
        "decoration-from-font" => "from-font",
    },
});

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "decoration",
    prop: SingleProp("text-decoration-thickness"),
})
.hints(&[PluginArbitraryHint::Length, PluginArbitraryHint::Percentage])
.matcher(OrMultiple(&[
    &Length,
    &Percentage,
    &CustomMultiple(&["auto", "from-font"]),
]));
