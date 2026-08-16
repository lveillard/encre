#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Spacing(Spacing {
    namespace: "indent",
    prop: SingleProp("text-indent"),
    ..Spacing::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "indent",
    prop: SingleProp("text-indent"),
    ..Arbitrary::default()
});
