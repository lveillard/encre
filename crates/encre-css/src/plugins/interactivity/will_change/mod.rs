#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("will-change"),
    values: phf_map! {
        "auto" => "auto",
        "scroll" => "scroll-position",
        "contents" => "contents",
        "transform" => "transform",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("will-change"),
    hints: &[],
    matcher: All,
});
