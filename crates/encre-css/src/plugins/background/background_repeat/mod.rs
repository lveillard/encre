#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-repeat"),
    values: phf_map! {
        "repeat" => "repeat",
        "no-repeat" => "no-repeat",
        "repeat-x" => "repeat-x",
        "repeat-y" => "repeat-y",
        "repeat-round" => "round",
        "repeat-space" => "space",
    },
});
