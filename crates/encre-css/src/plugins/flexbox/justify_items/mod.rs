#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("justify-items"),
    values: &["stretch", "start", "center", "end"],
});
