#![doc = include_str!("README.md")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListProperties {
    props: phf_map! {
        "break-normal" => &[
            "overflow-wrap: normal;",
            "word-break: normal;",
        ],
        "break-words" => &[
            "overflow-wrap: break-word;"
        ],
        "break-all" => &[
            "word-break: break-all;"
        ],
        "break-keep" => &[
            "word-break: keep-all;"
        ],
    },
});
