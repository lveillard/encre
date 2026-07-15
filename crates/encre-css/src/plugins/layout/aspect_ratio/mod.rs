#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("aspect-ratio"),
    values: phf_map! {
        "aspect-auto" => "auto",
        "aspect-square" => "1 / 1",
        "aspect-video" => "16 / 9",
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "aspect",
    prop: SingleProp("aspect-ratio"),
});
