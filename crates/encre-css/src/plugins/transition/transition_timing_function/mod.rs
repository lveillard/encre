#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("transition-timing-function"),
    values: map! {
        "ease-linear" => "linear",
        "ease-in" => "cubic-bezier(0.4, 0, 1, 1)",
        "ease-out" => "cubic-bezier(0, 0, 0.2, 1)",
        "ease-in-out" => "cubic-bezier(0.4, 0, 0.2, 1)",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "ease",
    prop: SingleProp("transition-timing-function"),
    ..Arbitrary::default()
});
