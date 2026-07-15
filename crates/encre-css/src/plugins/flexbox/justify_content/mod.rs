#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("justify-content"),
    values: phf_map! {
        "justify-start" => "flex-start",
        "justify-center" => "center",
        "justify-end" => "flex-end",
        "justify-between" => "space-between",
        "justify-around" => "space-around",
        "justify-evenly" => "space-evenly",
    },
});
