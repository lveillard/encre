#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("perspective-origin"),
    values: phf_map! {
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
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "perspective-origin",
    prop: SingleProp("perspective-origin"),
});
