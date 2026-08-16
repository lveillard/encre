#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("will-change"),
    values: map! {
        "will-change-auto" => "auto",
        "will-change-scroll" => "scroll-position",
        "will-change-contents" => "contents",
        "will-change-transform" => "transform",
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "will-change",
    prop: SingleProp("will-change"),
});
