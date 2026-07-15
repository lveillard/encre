#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("clear"),
    values: phf_map! {
        "clear-start" => "inline-start",
        "clear-end" => "inline-end",
        "clear-left" => "left",
        "clear-right" => "right",
        "clear-both" => "both",
        "clear-none" => "none",
    },
});
