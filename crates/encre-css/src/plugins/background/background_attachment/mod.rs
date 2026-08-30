#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("background-attachment"),
    values: map! {
        "bg-fixed" => "fixed",
        "bg-local" => "local",
        "bg-scroll" => "scroll",
    },
    ..ListValues::default()
});
