#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::SamePropValues {
    prop: SingleProp("caption-side"),
    values: &["top", "bottom"],
};
