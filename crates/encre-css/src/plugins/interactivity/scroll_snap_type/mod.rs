#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListCases {
    cases: phf_map! {
        "none" => &["-ms-scroll-snap-type: none;", "scroll-snap-type: none;"],
        "x" => &[
            "-ms-scroll-snap-type: x var(--en-scroll-snap-strictness);",
            "scroll-snap-type: x var(--en-scroll-snap-strictness);",
        ],
        "y" => &[
            "-ms-scroll-snap-type: y var(--en-scroll-snap-strictness);",
            "scroll-snap-type: y var(--en-scroll-snap-strictness);",
        ],
        "both" => &[
            "-ms-scroll-snap-type: both var(--en-scroll-snap-strictness);",
            "scroll-snap-type: both var(--en-scroll-snap-strictness);",
        ],
        "mandatory" => &["--en-scroll-snap-strictness: mandatory;"],
        "proximity" => &["--en-scroll-snap-strictness: proximity;"],
    },
};
