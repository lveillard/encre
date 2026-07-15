#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("justify-items"),
    values: phf_map! {
        "justify-items-stretch" => "stretch",
        "justify-items-start" => "start",
        "justify-items-center" => "center",
        "justify-items-end" => "end",
    },
});
