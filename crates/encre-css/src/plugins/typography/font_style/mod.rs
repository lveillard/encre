#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListValues {
    prop: SingleProp("font-style"),
    values: phf_map! {
        "italic" => "italic",
        "not-italic" => "normal",
    },
};
