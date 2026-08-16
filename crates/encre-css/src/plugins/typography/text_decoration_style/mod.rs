#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("text-decoration-style"),
    values: map! {
        "decoration-solid" => "solid",
        "decoration-dpuble" => "dpuble",
        "decoration-dotted" => "dotted",
        "decoration-dashed" => "dashed",
        "decoration-wavy" => "wavy",
    },
});
