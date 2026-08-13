#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::new(PluginKind::ListProperties {
    props: phf_map! {
        "line-clamp-none" => &[
            "-webkit-line-clamp: unset;"
        ]
    },
});

pub(crate) const PLUGIN_NUMBER: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "line-clamp",
    prop: SingleProp("-webkit-line-clamp"),
})
.extra_lines(&[
    "overflow: hidden;",
    "display: -webkit-box;",
    "-webkit-box-orient: vertical;",
]);
