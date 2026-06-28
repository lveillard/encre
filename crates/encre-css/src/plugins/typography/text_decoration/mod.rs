#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListValues {
    prop: MultipleProps(&["-webkit-text-decoration-line", "text-decoration-line"]),
    values: phf_map! {
        "underline" => "underline",
        "overline" => "overline",
        "line-through" => "line-through",
        "no-underline" => "none",
    },
};
