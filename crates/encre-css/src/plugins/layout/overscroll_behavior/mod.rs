#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::ListCases {
    cases: phf_map! {
        "auto" => &["overscroll-behavior: auto;"],
        "x-auto" => &["overscroll-behavior-x: auto;"],
        "y-auto" => &["overscroll-behavior-y: auto;"],
        "contain" => &["overscroll-behavior: contain;"],
        "x-contain" => &["overscroll-behavior-x: contain;"],
        "y-contain" => &["overscroll-behavior-y: contain;"],
        "none" => &["overscroll-behavior: none;"],
        "x-none" => &["overscroll-behavior-x: none;"],
        "y-none" => &["overscroll-behavior-y: none;"],
    },
};
