#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("perspective"),
    values: phf_map! {
        "dramatic" => "100px",
        "near" => "300px",
        "normal" => "500px",
        "midrange" => "800px",
        "distant" => "1200px",
        "none" => "none",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("perspective"),
});
