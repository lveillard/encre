#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("background-repeat"),
    values: map! {
        "bg-repeat" => "repeat",
        "bg-no-repeat" => "no-repeat",
        "bg-repeat-x" => "repeat-x",
        "bg-repeat-y" => "repeat-y",
        "bg-repeat-round" => "round",
        "bg-repeat-space" => "space",
    },
    ..ListValues::default()
});
