#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("text-wrap"),
    values: map! {
        "text-wrap" => "wrap",
        "text-nowrap" => "nowrap",
        "text-balance" => "balance",
        "text-pretty" => "pretty",
    },
});
