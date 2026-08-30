#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "z",
    prop: SingleProp("z-index"),
    has_negative: Some(true),
    has_auto: Some(true),
    ..Number::default()
});
