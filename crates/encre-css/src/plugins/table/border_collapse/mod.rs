#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("border-collapse"),
    values: map! {
        "border-collapse" => "collapse",
        "border-separate" => "separate",
    },
    ..ListValues::default()
});
