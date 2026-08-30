#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("border-style"),
    values: map! {
        "divide-solid" => "solid",
        "divide-dashed" => "dashed",
        "divide-dotted" => "dotted",
        "divide-double" => "double",
        "divide-none" => "none"
    },
    extra_class: Some(" > :not([hidden]) ~ :not([hidden])"),
    ..ListValues::default()
});
