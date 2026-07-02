#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("transform-origin"),
    values: phf_map! {
        "bottom" => "bottom",
        "center" => "center",
        "left" => "left",
        "bottom-left" => "bottom left",
        "top-left" => "top left",
        "right" => "right",
        "bottom-right" => "bottom right",
        "top-right" => "top right",
        "top" => "top",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("transform-origin"),
    hints: &[],
    matcher: Position,
});
