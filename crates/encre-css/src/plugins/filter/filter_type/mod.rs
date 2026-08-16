#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListProperties(ListProperties {
    props: map! {
        "filter" => &[CSS_FILTER],
        "filter-none" => &["filter: none;"],
    },
    ..ListProperties::default()
});
