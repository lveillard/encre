#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("pointer-events"),
    values: map! {
        "pointer-events-auto" => "auto",
        "pointer-events-none" => "none",
    },
    ..ListValues::default()
});
