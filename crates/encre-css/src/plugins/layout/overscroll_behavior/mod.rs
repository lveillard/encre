#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListProperties {
    props: map! {
        "overscroll-auto" => &["overscroll-behavior: auto;"],
        "overscroll-x-auto" => &["overscroll-behavior-x: auto;"],
        "overscroll-y-auto" => &["overscroll-behavior-y: auto;"],
        "overscroll-contain" => &["overscroll-behavior: contain;"],
        "overscroll-x-contain" => &["overscroll-behavior-x: contain;"],
        "overscroll-y-contain" => &["overscroll-behavior-y: contain;"],
        "overscroll-none" => &["overscroll-behavior: none;"],
        "overscroll-x-none" => &["overscroll-behavior-x: none;"],
        "overscroll-y-none" => &["overscroll-behavior-y: none;"],
    },
});
