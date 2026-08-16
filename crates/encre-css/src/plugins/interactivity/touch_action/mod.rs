#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("touch-action"),
    values: map! {
        "touch-auto" => "auto",
        "touch-pan-x" => "pan-x",
        "touch-pan-left" => "pan-left",
        "touch-pan-right" => "pan-right",
        "touch-pan-y" => "pan-y",
        "touch-pan-up" => "pan-up",
        "touch-pan-down" => "pan-down",
        "touch-pinch-zoom" => "pinch-zoom",
        "touch-manipulation" => "manipulation",
        "touch-none" => "none",
    },
    ..ListValues::default()
});
