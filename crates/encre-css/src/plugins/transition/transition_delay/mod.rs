#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "delay",
    prop: SingleProp("transition-delay"),
    template: Some("{}ms"),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "delay",
    prop: SingleProp("transition-delay"),
    ..Arbitrary::default()
});
