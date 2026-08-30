#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-auto-rows"),
    values: map! {
        "auto-rows-auto" => "auto",
        "auto-rows-min" => "min-content",
        "auto-rows-max" => "max-content",
        "auto-rows-fr" => "minmax(0, 1fr)"
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "auto-rows",
    prop: SingleProp("grid-auto-rows"),
    ..Arbitrary::default()
});
