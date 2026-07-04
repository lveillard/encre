#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("scroll-behavior"),
    values: phf_map! {
        "scroll-auto" => "auto",
        "scroll-smooth" => "smooth",
    },
});
