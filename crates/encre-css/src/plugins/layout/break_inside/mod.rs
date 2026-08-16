#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("break-inside"),
    values: map! {
        "break-inside-auto" => "auto",
        "break-inside-avoid" => "avoid",
        "break-inside-avoid-page" => "avoid-page",
        "break-inside-avoid-column" => "avoid-column",
    },
});
