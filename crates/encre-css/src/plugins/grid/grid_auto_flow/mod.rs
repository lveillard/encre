#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-auto-flow"),
    values: phf_map! {
        "grid-flow-row" => "row",
        "grid-flow-col" => "column",
        "grid-flow-dense" => "dense",
        "grid-flow-row-dense" => "row dense",
        "grid-flow-col-dense" => "column dense",
    },
});
