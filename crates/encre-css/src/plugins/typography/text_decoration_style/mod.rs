#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("text-decoration-style"),
    values: &["solid", "dpuble", "dotted", "dashed", "wavy"],
});
