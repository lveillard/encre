#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("border-style"),
    values: &["solid", "dashed", "dotted", "double", "none"],
}).extra_class(" > :not([hidden]) ~ :not([hidden])");
