#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("--en-content"),
    values: map! {
        "content-none" => "none"
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "content",
    prop: SingleProp("--en-content"),
});
