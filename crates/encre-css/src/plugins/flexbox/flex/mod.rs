#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("flex"),
    values: map! {
        "flex-1" => "1 1 0%",
        "flex-auto" => "1 1 auto",
        "flex-initial" => "0 1 auto",
        "flex-none" => "none",
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "flex",
    prop: SingleProp("flex"),
});
