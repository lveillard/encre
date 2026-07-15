#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("place-items"),
    values: phf_map! {
        "place-items-stretch" => "stretch",
        "place-items-start" => "start",
        "place-items-center" => "center",
        "place-items-end" => "end",
    },
});
