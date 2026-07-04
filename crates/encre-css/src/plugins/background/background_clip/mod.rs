#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-clip"),
    values: phf_map! {
        "bg-clip-border" => "border-box",
        "bg-clip-padding" => "padding-box",
        "bg-clip-content" => "content-box",
        "bg-clip-text" => "text",
    },
});
