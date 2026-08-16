#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("flex-direction"),
    values: map! {
        "flex-row" => "row",
        "flex-row-reverse" => "row-reverse",
        "flex-col" => "column",
        "flex-col-reverse" => "column-reverse",
    },
    ..ListValues::default()
});
