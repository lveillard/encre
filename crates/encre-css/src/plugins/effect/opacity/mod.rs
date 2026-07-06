#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
    prefix: "opacity",
    prop: SingleProp("opacity"),
    has_empty: false,
    has_negative: false,
    divide_by: 100.0,
});
