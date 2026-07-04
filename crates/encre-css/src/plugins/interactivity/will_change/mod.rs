#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("will-change"),
    values: phf_map! {
        "will-change-auto" => "auto",
        "will-change-scroll" => "scroll-position",
        "will-change-contents" => "contents",
        "will-change-transform" => "transform",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "will-change",
    prop: SingleProp("will-change"),
});
