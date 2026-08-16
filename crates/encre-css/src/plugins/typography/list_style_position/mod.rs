#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("list-style-position"),
    values: map! {
        "list-inside" => "inside",
        "list-outside" => "outside",
    },
    ..ListValues::default()
});
