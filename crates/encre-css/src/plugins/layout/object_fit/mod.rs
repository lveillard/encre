#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("object-fit"),
    values: map! {
        "object-contain" => "contain",
        "object-cover" => "cover",
        "object-fill" => "fill",
        "object-scale-down" => "scale-down",
        "object-none" => "none",
    },
});
