#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_LIST_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("outline-style"),
    values: phf_map! {
        "outline-dashed" => "dashed",
        "outline-dotted" => "dotted",
        "outline-double" => "double",
    },
});

pub(crate) const PLUGIN_LIST_2: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "outline-hidden" => &[
            "outline: 2px solid transparent;",
            "outline-offset: 2px;"
        ],
    },
});
