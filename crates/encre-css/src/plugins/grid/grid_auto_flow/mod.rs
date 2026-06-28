#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-auto-flow"),
    values: phf_map! {
        "row" => "row",
        "col" => "column",
        "dense" => "dense",
        "row-dense" => "row dense",
        "col-dense" => "column dense",
    },
});
