#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("align-self"),
    values: map! {
        "self-auto" => "auto",
        "self-start" => "flex-start",
        "self-center" => "center",
        "self-end" => "flex-end",
        "self-stretch" => "stretch",
    },
});
