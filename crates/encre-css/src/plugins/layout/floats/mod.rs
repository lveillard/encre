#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("float"),
    values: map! {
        "float-start" => "inline-start",
        "float-end" => "inline-end",
        "float-left" => "left",
        "float-right" => "right",
        "float-none" => "none",
    },
    ..ListValues::default()
});
