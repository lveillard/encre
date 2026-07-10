#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
    namespace: "z",
    prop: SingleProp("z-index"),
    divide_by: 1.0,
}).has_negative().has_auto();
