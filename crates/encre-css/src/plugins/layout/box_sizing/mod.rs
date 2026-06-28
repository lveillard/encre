#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListValues {
    prop: SingleProp("box-sizing"),
    values: phf_map! {
        "border" => "border-box",
        "content" => "content-box",
    },
};
