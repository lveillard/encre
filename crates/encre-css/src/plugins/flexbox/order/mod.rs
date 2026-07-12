#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_LIST: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("order"),
    values: phf_map! {
        "order-first" => "-9999",
        "order-last" => "9999",
        "order-none" => "0",
    },
});

pub(crate) const PLUGIN_NUM: Plugin = Plugin::new(PluginKind::Number {
    namespace: "order",
    prop: SingleProp("order"),
    divide_by: 1.0,
}).has_negative();
