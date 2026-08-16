#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "shrink",
    prop: SingleProp("flex-shrink"),
    has_empty: Some(true),
    ..Number::default()
});
