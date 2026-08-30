#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("aspect-ratio"),
    values: map! {
        "aspect-auto" => "auto",
        "aspect-square" => "1 / 1",
        "aspect-video" => "16 / 9",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "aspect",
    prop: SingleProp("aspect-ratio"),
    ..Arbitrary::default()
});
