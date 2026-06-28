#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("isolation"),
    values: phf_map! {
        "isolate" => "isolate",
        "isolation-auto" => "auto",
    },
});
