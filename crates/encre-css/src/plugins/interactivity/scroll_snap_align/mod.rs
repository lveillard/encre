#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("scroll-snap-align"),
    values: phf_map! {
        "snap-start" => "start",
        "snap-end" => "end",
        "snap-center" => "center",
        "snap-align-none" => "none",
    },
});
