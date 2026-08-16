#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("white-space"),
    values: map! {
        "whitespace-normal" => "normal",
        "whitespace-nowrap" => "nowrap",
        "whitespace-pre" => "pre",
        "whitespace-pre-line" => "pre-line",
        "whitespace-pre-wrap" => "pre-wrap",
        "whitespace-break-spaces" => "break-spaces",
    },
    ..ListValues::default()
});
