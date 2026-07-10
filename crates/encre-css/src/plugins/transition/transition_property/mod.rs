#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "transition" => &[
            "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
        "transition-none" => &["transition-property: none;"],
        "transition-all" => &[
            "transition-property: all;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
        "transition-colors" => &[
            "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
        "transition-opacity" => &[
            "transition-property: opacity;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
        "transition-shadow" => &[
            "transition-property: box-shadow;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
        "transition-transform" => &[
            "transition-property: transform;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "transition",
    prop: SingleProp("transition-property"),
});
