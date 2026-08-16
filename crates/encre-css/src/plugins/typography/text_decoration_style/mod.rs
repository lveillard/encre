#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("text-decoration-style"),
    values: map! {
        "decoration-solid" => "solid",
        "decoration-dpuble" => "dpuble",
        "decoration-dotted" => "dotted",
        "decoration-dashed" => "dashed",
        "decoration-wavy" => "wavy",
    },
    ..ListValues::default()
});
