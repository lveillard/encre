#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::ListValues {
    prop: SingleProp("perspective-origin"),
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
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("perspective-origin"),
    hints: &[PluginArbitraryHint::Position],
    matcher: Position,
};
