#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("table-layout"),
    values: map! {
        "table-auto" => "auto",
        "table-fixed" => "fixed",
    },
});
