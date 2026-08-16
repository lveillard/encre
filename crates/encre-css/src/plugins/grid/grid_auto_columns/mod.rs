#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("grid-auto-columns"),
    values: map! {
        "auto-cols-auto" => "auto",
        "auto-cols-min" => "min-content",
        "auto-cols-max" => "max-content",
        "auto-cols-fr" => "minmax(0, 1fr)"
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "auto-cols",
    prop: SingleProp("grid-auto-columns"),
    ..Arbitrary::default()
});
