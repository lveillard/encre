#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-origin"),
    values: phf_map! {
        "bg-origin-border" => "border-box",
        "bg-origin-padding" => "padding-box",
        "bg-origin-content" => "content-box",
    },
});
