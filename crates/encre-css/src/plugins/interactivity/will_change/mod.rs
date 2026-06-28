#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::ListValues {
    prop: SingleProp("will-change"),
    values: phf_map! {
        "auto" => "auto",
        "scroll" => "scroll-position",
        "contents" => "contents",
        "transform" => "transform",
    },
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("will-change"),
    hints: &[],
    matcher: All,
};
