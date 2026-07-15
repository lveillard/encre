#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "snap-none" => &["-ms-scroll-snap-type: none;", "scroll-snap-type: none;"],
        "snap-x" => &[
            "-ms-scroll-snap-type: x var(--en-scroll-snap-strictness);",
            "scroll-snap-type: x var(--en-scroll-snap-strictness);",
        ],
        "snap-y" => &[
            "-ms-scroll-snap-type: y var(--en-scroll-snap-strictness);",
            "scroll-snap-type: y var(--en-scroll-snap-strictness);",
        ],
        "snap-both" => &[
            "-ms-scroll-snap-type: both var(--en-scroll-snap-strictness);",
            "scroll-snap-type: both var(--en-scroll-snap-strictness);",
        ],
        "snap-mandatory" => &["--en-scroll-snap-strictness: mandatory;"],
        "snap-proximity" => &["--en-scroll-snap-strictness: proximity;"],
    },
});
