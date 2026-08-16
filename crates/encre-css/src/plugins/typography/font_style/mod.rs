#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("font-style"),
    values: map! {
        "italic" => "italic",
        "not-italic" => "normal",
    },
    ..ListValues::default()
});
