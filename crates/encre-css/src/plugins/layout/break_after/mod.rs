#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("break-after"),
    values: map! {
        "break-after-auto" => "auto",
        "break-after-avoid" => "avoid",
        "break-after-all" => "all",
        "break-after-avoid-page" => "avoid-page",
        "break-after-page" => "page",
        "break-after-left" => "left",
        "break-after-right" => "right",
        "break-after-column" => "column",
    },
    ..ListValues::default()
});
