#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_NUMBER: Plugin = Plugin::new(PluginKind::Number {
    namespace: "decoration",
    prop: SingleProp("text-decoration-thickness"),
    divide_by: 1.0,
})
.template("{}px");

pub(crate) const PLUGIN_LIST: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("text-decoration-thickness"),
    values: phf_map! {
        "decoration-auto" => "auto",
        "decoration-from-font" => "from-font",
    },
});

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "decoration",
    prop: SingleProp("text-decoration-thickness"),
})
.hints(&[ArbitraryHint::Length, ArbitraryHint::Percentage])
.matcher(OrMultiple(&[
    &Length,
    &Percentage,
    &CustomMultiple(&["auto", "from-font"]),
]));
