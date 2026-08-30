#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("order"),
    values: map! {
        "order-first" => "-9999",
        "order-last" => "9999",
        "order-none" => "0",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_NUM: StaticPlugin = Plugin::Number(Number {
    namespace: "order",
    prop: SingleProp("order"),
    has_negative: Some(true),
    ..Number::default()
});
