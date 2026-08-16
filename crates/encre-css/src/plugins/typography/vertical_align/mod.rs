#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("vertical-align"),
    values: map! {
        "align-baseline" => "baseline",
        "align-top" => "top",
        "align-middle" => "middle",
        "align-bottom" => "bottom",
        "align-text-top" => "text-top",
        "align-text-bottom" => "text-bottom",
        "align-sub" => "sub",
        "align-super" => "super",
    },
});
