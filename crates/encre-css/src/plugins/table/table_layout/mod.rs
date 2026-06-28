#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("table-layout"),
    values: &["auto", "fixed"],
});
