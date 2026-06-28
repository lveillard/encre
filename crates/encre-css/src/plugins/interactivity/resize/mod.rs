#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListValues {
    prop: SingleProp("resize"),
    values: phf_map! {
        "" => "both",
        "none" => "none",
        "x" => "horizontal",
        "y" => "vertical",
    },
};
