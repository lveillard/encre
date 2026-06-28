#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListValues {
    prop: SingleProp("float"),
    values: phf_map! {
        "start" => "inline-start",
        "end" => "inline-end",
        "left" => "left",
        "right" => "right",
        "none" => "none",
    },
};
