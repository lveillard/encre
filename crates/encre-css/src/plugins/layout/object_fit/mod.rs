#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::SamePropValues {
    prop: SingleProp("object-fit"),
    values: &["contain", "cover", "fill", "scale-down", "none"],
};
