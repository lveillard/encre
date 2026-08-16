#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("user-select"),
    values: map! {
        "select-text" => "text",
        "select-all" => "all",
        "select-auto" => "auto",
        "select-none" => "none",
    },
});
