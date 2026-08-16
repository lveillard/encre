#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("transform-origin"),
    values: map! {
        "origin-bottom" => "bottom",
        "origin-center" => "center",
        "origin-left" => "left",
        "origin-bottom-left" => "bottom left",
        "origin-top-left" => "top left",
        "origin-right" => "right",
        "origin-bottom-right" => "bottom right",
        "origin-top-right" => "top right",
        "origin-top" => "top",
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "origin",
    prop: SingleProp("transform-origin"),
});
