#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::AnyNumber {
    prefix: "grid-cols",
    prop: SingleProp("grid-template-columns"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
})
.template("repeat({}, minmax(0, 1fr))");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-template-columns"),
    values: phf_map! {
        "grid-cols-none" => "none",
    },
});

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "grid-cols",
    prop: SingleProp("grid-template-columns"),
});
