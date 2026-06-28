#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("place-content"),
    values: phf_map! {
        "start" => "start",
        "center" => "center",
        "end" => "end",
        "between" => "space-between",
        "around" => "space-around",
        "evenly" => "space-evenly",
    },
});
