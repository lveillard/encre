#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("pointer-events"),
    values: phf_map! {
        "pointer-events-auto" => "auto",
        "pointer-events-none" => "none",
    },
});
