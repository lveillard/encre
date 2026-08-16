#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("will-change"),
    values: map! {
        "will-change-auto" => "auto",
        "will-change-scroll" => "scroll-position",
        "will-change-contents" => "contents",
        "will-change-transform" => "transform",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "will-change",
    prop: SingleProp("will-change"),
    ..Arbitrary::default()
});
