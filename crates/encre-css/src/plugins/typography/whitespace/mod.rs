#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::SamePropValues {
    prop: SingleProp("white-space"),
    values: &[
        "normal",
        "nowrap",
        "pre",
        "pre-line",
        "pre-wrap",
        "break-spaces",
    ],
};
