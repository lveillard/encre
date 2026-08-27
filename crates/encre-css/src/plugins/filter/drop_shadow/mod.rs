#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("--en-drop-shadow"),
    values: map! {
        "drop-shadow-xs" => "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))",
        "drop-shadow-sm" => "drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))",
        "drop-shadow-md" => "drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))",
        "drop-shadow-lg" => "drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))",
        "drop-shadow-xl" => "drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))",
        "drop-shadow-2xl" => "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))",
        "drop-shadow-none" => "drop-shadow(0 0 #0000)",
    },
    extra_rule_css: Some(&[CSS_FILTER]),
    ..ListValues::default()
});

// TODO(in tailwindcss v4.3): support changing drop shadow color with class + replace color by variable in arbitrary plugin like in box_shadow
pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "drop-shadow",
    prop: SingleProp("--en-drop-shadow"),
    extra_rule_css: Some(&[CSS_FILTER]),
    template: Some("drop-shadow({})"),
    ..Arbitrary::default()
});
