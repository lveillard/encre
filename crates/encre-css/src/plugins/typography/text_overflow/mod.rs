#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListCases {
    cases: phf_map! {
        "truncate" => &[
            "overflow: hidden;",
            "text-overflow: ellipsis;",
            "white-space: nowrap;",
        ],
        "text-ellipsis" => &[
            "text-overflow: ellipsis;"
        ],
        "text-clip" => &[
            "text-overflow: clip;"
        ],
    },
};
