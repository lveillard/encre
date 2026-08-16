#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListProperties {
    props: map! {
        "overflow-auto" => &["overflow: auto;"],
        "overflow-x-auto" => &["overflow-x: auto;"],
        "overflow-y-auto" => &["overflow-y: auto;"],
        "overflow-hidden" => &["overflow: hidden;"],
        "overflow-x-hidden" => &["overflow-x: hidden;"],
        "overflow-y-hidden" => &["overflow-y: hidden;"],
        "overflow-visible" => &["overflow: visible;"],
        "overflow-x-visible" => &["overflow-x: visible;"],
        "overflow-y-visible" => &["overflow-y: visible;"],
        "overflow-scroll" => &["overflow: scroll;"],
        "overflow-x-scroll" => &["overflow-x: scroll;"],
        "overflow-y-scroll" => &["overflow-y: scroll;"],
    },
});
