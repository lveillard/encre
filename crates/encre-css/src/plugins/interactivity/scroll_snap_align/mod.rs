#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues { prop: SingleProp("scroll-snap-align"), values: phf_map! {
    "start" => "start",
    "end" => "end",
    "center" => "center",
    "align-none" => "none",
} });
