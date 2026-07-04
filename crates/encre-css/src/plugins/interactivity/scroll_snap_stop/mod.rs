#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("scroll-snap-stop"),
    values: phf_map! {
        "snap-normal" => "normal",
        "snap-always" => "always",
    },
});
