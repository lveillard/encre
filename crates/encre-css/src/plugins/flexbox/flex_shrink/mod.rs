#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "shrink",
    prop: SingleProp("flex-shrink"),
    divide_by: 1.0,
}).has_empty();
