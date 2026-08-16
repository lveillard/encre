#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("align-content"),
    values: map! {
        "content-start" => "flex-start",
        "content-center" => "center",
        "content-end" => "flex-end",
        "content-between" => "space-between",
        "content-around" => "space-around",
        "content-evenly" => "space-evenly",
    },
});
