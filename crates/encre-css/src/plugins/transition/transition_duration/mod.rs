#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Number(Number {
    namespace: "duration",
    prop: SingleProp("transition-duration"),
    template: Some(SingleProp("{}ms")),
    ..Number::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "duration",
    prop: SingleProp("transition-duration"),
    ..Arbitrary::default()
});
