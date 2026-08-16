#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListProperties(ListProperties {
    props: map! {
        "backdrop-filter" => &CSS_BACKDROP_FILTER,
        "backdrop-filter-none" => &[
            "-webkit-backdrop-filter: none;",
            "backdrop-filter: none;",
        ],
    },
    ..ListProperties::default()
});
