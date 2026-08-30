#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("text-align"),
    values: map! {
        "text-left" => "left",
        "text-center" => "center",
        "text-right" => "right",
        "text-justify" => "justify",
        "text-start" => "start",
        "text-end" => "end",
    },
    ..ListValues::default()
});
