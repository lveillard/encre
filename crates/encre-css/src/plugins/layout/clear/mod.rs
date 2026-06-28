#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("clear"),
    values: phf_map! {
        "start" => "inline-start",
        "end" => "inline-end",
        "left" => "left",
        "right" => "right",
        "both" => "both",
        "none" => "none",
    },
});
