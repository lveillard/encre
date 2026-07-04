#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("flex"),
    values: phf_map! {
        "flex-1" => "1 1 0%",
        "flex-auto" => "1 1 auto",
        "flex-initial" => "0 1 auto",
        "flex-none" => "none",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "flex",
    prop: SingleProp("flex"),
});
