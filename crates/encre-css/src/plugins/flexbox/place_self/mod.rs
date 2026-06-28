#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListValues {
    prop: SingleProp("place-self"),
    values: phf_map! {
        "auto" => "auto",
        "start" => "flex-start",
        "center" => "center",
        "end" => "flex-end",
        "stretch" => "stretch",
    },
};
