#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("user-select"),
    values: map! {
        "select-text" => "text",
        "select-all" => "all",
        "select-auto" => "auto",
        "select-none" => "none",
    },
    ..ListValues::default()
});
