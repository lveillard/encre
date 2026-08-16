#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("--en-blur"),
    values: map! {
        "blur-xs" => "blur(4px)",
        "blur-sm" => "blur(8px)",
        "blur-md" => "blur(12px)",
        "blur-lg" => "blur(16px)",
        "blur-xl" => "blur(24px)",
        "blur-2xl" => "blur(40px)",
        "blur-3xl" => "blur(64px)",
        "blur-none" => "blur(0)",
    },
    extra_lines: Some(&[CSS_FILTER]),
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "blur",
    prop: SingleProp("--en-blur"),
    extra_lines: Some(&[CSS_FILTER]),
    template: Some("blur({})"),
    ..Arbitrary::default()
});
