#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("align-items"),
    values: phf_map! {
        "items-stretch" => "stretch",
        "items-start" => "flex-start",
        "items-center" => "center",
        "items-end" => "flex-end",
        "items-baseline" => "baseline",
    },
});
