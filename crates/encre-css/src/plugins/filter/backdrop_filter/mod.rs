#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListProperties {
    props: phf_map! {
        "backdrop-filter" => &CSS_BACKDROP_FILTER,
        "backdrop-filter-none" => &[
            "-webkit-backdrop-filter: none;",
            "backdrop-filter: none;",
        ],
    },
});
