#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("perspective"),
    values: phf_map! {
        "perspective-dramatic" => "100px",
        "perspective-near" => "300px",
        "perspective-normal" => "500px",
        "perspective-midrange" => "800px",
        "perspective-distant" => "1200px",
        "perspective-none" => "none",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "perspective",
    prop: SingleProp("perspective"),
});
