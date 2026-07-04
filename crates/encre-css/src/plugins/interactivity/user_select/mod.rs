#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("user-select"),
    values: phf_map! {
        "select-text" => "text",
        "select-all" => "all",
        "select-auto" => "auto",
        "select-none" => "none",
    },
});
