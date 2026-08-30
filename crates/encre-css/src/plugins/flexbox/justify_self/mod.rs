#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("justify-self"),
    values: map! {
        "justify-self-auto" => "auto",
        "justify-self-start" => "flex-start",
        "justify-self-center" => "center",
        "justify-self-end" => "flex-end",
        "justify-self-stretch" => "stretch",
    },
    ..ListValues::default()
});
