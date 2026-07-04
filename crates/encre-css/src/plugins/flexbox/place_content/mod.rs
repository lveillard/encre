#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("place-content"),
    values: phf_map! {
        "place-content-start" => "start",
        "place-content-center" => "center",
        "place-content-end" => "end",
        "place-content-between" => "space-between",
        "place-content-around" => "space-around",
        "place-content-evenly" => "space-evenly",
    },
});
