#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("z-index"),
    values: phf_map! {
        "z-auto" => "auto",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Number {
    prefix: "z",
    prop: SingleProp("z-index"),
    has_empty: false,
    has_negative: true,
    divide_by: 1.0,
});
