#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("text-align"),
    values: phf_map! {
        "text-left" => "left",
        "text-center" => "center",
        "text-right" => "right",
        "text-justify" => "justify",
        "text-start" => "start",
        "text-end" => "end",
    },
});
