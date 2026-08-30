#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::ListProperties(ListProperties {
    props: map! {
        "line-clamp-none" => &[
            "-webkit-line-clamp: unset;"
        ]
    },
    ..ListProperties::default()
});

pub(crate) const PLUGIN_NUMBER: StaticPlugin = Plugin::Number(Number {
    namespace: "line-clamp",
    prop: SingleProp("-webkit-line-clamp"),
    extra_rule_css: Some(&[
        "overflow: hidden;",
        "display: -webkit-box;",
        "-webkit-box-orient: vertical;",
    ]),
    ..Number::default()
});
