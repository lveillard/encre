#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Spacing {
    namespace: "basis",
    prop: SingleProp("flex-basis"),
}).has_auto().has_full();

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "basis",
    prop: SingleProp("flex-basis"),
});
