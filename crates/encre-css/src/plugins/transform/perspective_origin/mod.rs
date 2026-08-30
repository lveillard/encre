#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("perspective-origin"),
    values: map! {
        "perspective-origin-bottom" => "bottom",
        "perspective-origin-center" => "center",
        "perspective-origin-left" => "left",
        "perspective-origin-bottom-left" => "bottom left",
        "perspective-origin-top-left" => "top left",
        "perspective-origin-right" => "right",
        "perspective-origin-bottom-right" => "bottom right",
        "perspective-origin-top-right" => "top right",
        "perspective-origin-top" => "top",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "perspective-origin",
    prop: SingleProp("perspective-origin"),
    ..Arbitrary::default()
});
