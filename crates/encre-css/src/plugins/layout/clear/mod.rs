#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("clear"),
    values: map! {
        "clear-start" => "inline-start",
        "clear-end" => "inline-end",
        "clear-left" => "left",
        "clear-right" => "right",
        "clear-both" => "both",
        "clear-none" => "none",
    },
    ..ListValues::default()
});
