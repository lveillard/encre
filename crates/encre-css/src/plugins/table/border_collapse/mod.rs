#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("border-collapse"),
    values: phf_map! {
        "border-collapse" => "collapse",
        "border-separate" => "separate",
    },
});
