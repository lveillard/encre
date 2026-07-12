#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("container-type"),
    values: phf_map! {
        "@container" => "inline-size",
    },
});

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "@container",
    prop: SingleProp("container-type"),
});
