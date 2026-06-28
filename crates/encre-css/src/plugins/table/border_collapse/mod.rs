#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::SamePropValues {
    prop: SingleProp("border-collapse"),
    values: &["collapse", "separate"],
};
