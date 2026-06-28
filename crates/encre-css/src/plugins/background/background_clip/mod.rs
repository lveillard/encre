#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-clip"),
    values: phf_map! {
        "border" => "border-box",
        "padding" => "padding-box",
        "content" => "content-box",
        "text" => "text",
    },
});
