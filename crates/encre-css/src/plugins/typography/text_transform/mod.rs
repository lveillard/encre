#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("text-transform"),
    values: phf_map! {
        "uppercase" => "uppercase",
        "lowercase" => "lowercase",
        "capitalize" => "capitalize",
        "normal-case" => "none",
    },
});
