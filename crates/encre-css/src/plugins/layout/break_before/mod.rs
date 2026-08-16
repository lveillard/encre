#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("break-before"),
    values: map! {
        "break-before-auto" => "auto",
        "break-before-avoid" => "avoid",
        "break-before-all" => "all",
        "break-before-avoid-page" => "avoid-page",
        "break-before-page" => "page",
        "break-before-left" => "left",
        "break-before-right" => "right",
        "break-before-column" => "column",
    },
});
