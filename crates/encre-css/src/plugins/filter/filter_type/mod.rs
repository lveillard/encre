#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListProperties {
    props: phf_map! {
        "filter" => &[CSS_FILTER],
        "filter-none" => &["filter: none;"],
    },
});
