#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "auto" => &["overflow: auto;"],
        "x-auto" => &["overflow-x: auto;"],
        "y-auto" => &["overflow-y: auto;"],
        "hidden" => &["overflow: hidden;"],
        "x-hidden" => &["overflow-x: hidden;"],
        "y-hidden" => &["overflow-y: hidden;"],
        "visible" => &["overflow: visible;"],
        "x-visible" => &["overflow-x: visible;"],
        "y-visible" => &["overflow-y: visible;"],
        "scroll" => &["overflow: scroll;"],
        "x-scroll" => &["overflow-x: scroll;"],
        "y-scroll" => &["overflow-y: scroll;"],
    },
});
