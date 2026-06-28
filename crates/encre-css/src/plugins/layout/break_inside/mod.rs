#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("break-inside"),
    values: &["auto", "avoid", "avoid-page", "avoid-column"],
});
