#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::SamePropValues {
    prop: SingleProp("scroll-snap-stop"),
    values: &["normal", "always"],
};
