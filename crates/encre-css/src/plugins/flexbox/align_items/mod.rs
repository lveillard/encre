#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListValues {
    prop: SingleProp("align-items"),
    values: phf_map! {
        "stretch" => "stretch",
        "start" => "flex-start",
        "center" => "center",
        "end" => "flex-end",
        "baseline" => "baseline",
    },
};
