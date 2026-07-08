#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "line-clamp-none" => &[
            "-webkit-line-clamp: unset;"
        ]
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Number {
    prefix: "line-clamp",
    prop: SingleProp("-webkit-line-clamp"),
    divide_by: 1.0,
})
.extra_lines(&[
    "overflow: hidden;",
    "display: -webkit-box;",
    "-webkit-box-orient: vertical;",
]);
