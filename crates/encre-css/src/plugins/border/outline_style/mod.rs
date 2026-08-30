#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_LIST_1: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("outline-style"),
    values: map! {
        "outline-dashed" => "dashed",
        "outline-dotted" => "dotted",
        "outline-double" => "double",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_LIST_2: StaticPlugin = Plugin::ListProperties(ListProperties {
    props: map! {
        "outline-hidden" => &[
            "outline: 2px solid transparent;",
            "outline-offset: 2px;"
        ],
    },
    ..ListProperties::default()
});
