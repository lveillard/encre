#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("resize"),
    values: phf_map! {
        "resize" => "both",
        "resize-none" => "none",
        "resize-x" => "horizontal",
        "resize-y" => "vertical",
    },
});
