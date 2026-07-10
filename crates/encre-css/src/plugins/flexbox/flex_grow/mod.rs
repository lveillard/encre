#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
    namespace: "grow",
    prop: SingleProp("flex-grow"),
    divide_by: 1.0,
}).has_empty();
