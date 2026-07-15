#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("flex-direction"),
    values: phf_map! {
        "flex-row" => "row",
        "flex-row-reverse" => "row-reverse",
        "flex-col" => "column",
        "flex-col-reverse" => "column-reverse",
    },
});
