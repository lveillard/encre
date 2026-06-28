#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("align-content"),
    values: phf_map! {
        "start" => "flex-start",
        "center" => "center",
        "end" => "flex-end",
        "between" => "space-between",
        "around" => "space-around",
        "evenly" => "space-evenly",
    },
});
