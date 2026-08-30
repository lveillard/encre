#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Color(Color {
    namespace: "fill",
    prop: SingleProp("fill"),
    ..Color::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "fill",
    prop: SingleProp("fill"),
    ..Arbitrary::default()
});
