#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("object-position"),
    values: map! {
        "object-bottom" => "bottom",
        "object-center" => "center",
        "object-left" => "left",
        "object-bottom-left" => "bottom left",
        "object-top-left" => "top left",
        "object-right" => "right",
        "object-bottom-right" => "bottom right",
        "object-top-right" => "top right",
        "object-top" => "top",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "object",
    prop: SingleProp("object-position"),
    ..Arbitrary::default()
});
