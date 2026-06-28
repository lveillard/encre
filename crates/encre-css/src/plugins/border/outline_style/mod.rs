#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("outline-style"),
    values: &["dashed", "dotted", "double"],
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "hidden" => &[
            "outline: 2px solid transparent;",
            "outline-offset: 2px;"
        ],
    },
});
