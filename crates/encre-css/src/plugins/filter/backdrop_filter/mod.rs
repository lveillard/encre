#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "" => &CSS_BACKDROP_FILTER,
        "none" => &[
            "-webkit-backdrop-filter: none;",
            "backdrop-filter: none;",
        ],
    },
});
