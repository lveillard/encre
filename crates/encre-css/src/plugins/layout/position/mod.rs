#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("position"),
    values: map! {
        "static" => "static",
        "fixed" => "fixed",
        "absolute" => "absolute",
        "relative" => "relative",
        "sticky" => "sticky",
    },
});
