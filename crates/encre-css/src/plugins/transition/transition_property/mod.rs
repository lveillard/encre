#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::ListCases {
    cases: phf_map! {
        "" => &[
            "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
        "none" => &["transition-property: none;"],
        "all" => &[
            "transition-property: all;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
        "colors" => &[
            "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
        "opacity" => &[
            "transition-property: opacity;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
        "shadow" => &[
            "transition-property: box-shadow;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
        "transform" => &[
            "transition-property: transform;",
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
            "transition-duration: 150ms;",
        ],
    },
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("transition-property"),
    hints: &[],
    matcher: All,
};
