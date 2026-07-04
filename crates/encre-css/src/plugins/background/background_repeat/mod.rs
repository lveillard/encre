#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-repeat"),
    values: phf_map! {
        "bg-repeat" => "repeat",
        "bg-no-repeat" => "no-repeat",
        "bg-repeat-x" => "repeat-x",
        "bg-repeat-y" => "repeat-y",
        "bg-repeat-round" => "round",
        "bg-repeat-space" => "space",
    },
});
