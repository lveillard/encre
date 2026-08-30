#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("list-style-type"),
    values: map! {
        "list-disc" => "disc",
        "list-decimal" => "decimal",
        "list-none" => "none",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "list",
    prop: SingleProp("list-style-type"),
    ..Arbitrary::default()
});
