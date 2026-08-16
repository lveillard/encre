#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("box-decoration-break"),
    values: map! {
        "box-decoration-slice" => "slice",
        "box-decoration-clone" => "clone",
    },
});
