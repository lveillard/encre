#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-attachment"),
    values: phf_map! {
        "bg-fixed" => "fixed",
        "bg-local" => "local",
        "bg-scroll" => "scroll",
    },
});
