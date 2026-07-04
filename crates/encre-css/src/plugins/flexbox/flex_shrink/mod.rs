#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::AnyNumber {
    prefix: "shrink",
    prop: SingleProp("flex-shrink"),
    has_empty: true,
    has_negative: false,
    divide_by: 1.0,
});
