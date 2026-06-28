#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListValues {
    prop: SingleProp("flex-direction"),
    values: phf_map! {
        "row" => "row",
        "row-reverse" => "row-reverse",
        "col" => "column",
        "col-reverse" => "column-reverse",
    },
};
