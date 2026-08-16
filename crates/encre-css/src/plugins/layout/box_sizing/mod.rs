#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("box-sizing"),
    values: map! {
        "box-border" => "border-box",
        "box-content" => "content-box",
    },
});
