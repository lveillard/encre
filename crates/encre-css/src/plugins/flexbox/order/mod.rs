#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("order"),
    values: phf_map! {
        "order-first" => "-9999",
        "order-last" => "9999",
        "order-none" => "0",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Number {
    prefix: "order",
    prop: SingleProp("order"),
    has_empty: false,
    has_negative: true,
    divide_by: 1.0,
});
