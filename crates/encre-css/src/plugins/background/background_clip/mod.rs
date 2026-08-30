#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("background-clip"),
    values: map! {
        "bg-clip-border" => "border-box",
        "bg-clip-padding" => "padding-box",
        "bg-clip-content" => "content-box",
        "bg-clip-text" => "text",
    },
    ..ListValues::default()
});
