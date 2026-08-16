#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("caption-side"),
    values: map! {
        "caption-top" => "top",
        "caption-bottom" => "bottom",
    },
});
