#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("order"),
    values: phf_map! {
        "first" => "-9999",
        "last" => "9999",
        "none" => "0",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("order"),
    has_empty: false,
    has_negative: true,
    divide_by: 1.0,
});
