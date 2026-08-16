#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("flex-wrap"),
    values: map! {
        "flex-nowrap" => "nowrap",
        "flex-wrap" => "wrap",
        "flex-wrap-reverse" => "wrap-reverse",
    },
    ..ListValues::default()
});
