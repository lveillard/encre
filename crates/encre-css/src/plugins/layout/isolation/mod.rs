#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("isolation"),
    values: map! {
        "isolate" => "isolate",
        "isolation-auto" => "auto",
    },
    ..ListValues::default()
});
