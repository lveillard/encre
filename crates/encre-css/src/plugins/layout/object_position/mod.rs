#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("object-position"),
    values: phf_map! {
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
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "object",
    prop: SingleProp("object-position"),
});
