#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "opacity",
    prop: SingleProp("opacity"),
    divide_by: Some(100.0),
    ..Number::default()
});
