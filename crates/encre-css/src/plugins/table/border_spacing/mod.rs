#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

const fn builtin_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Spacing {
        prefix,
        prop,
        has_auto: false,
        has_full: false,
    })
    .extra_lines(&["border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);"])
}

const fn arbitrary_plugin(prefix: &'static str, prop: PropertyName) -> Plugin {
    Plugin::new(PluginKind::Arbitrary {
        prefix,
        prop,
    })
    .extra_lines(&["border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);"])
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin("border-spacing", MultipleProps(&[
    "--en-border-spacing-x",
    "--en-border-spacing-y",
]));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin("border-spacing", MultipleProps(&[
    "--en-border-spacing-x",
    "--en-border-spacing-y",
]));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin("border-spacing-x", SingleProp("--en-border-spacing-x"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin("border-spacing-x", SingleProp("--en-border-spacing-x"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin("border-spacing-y", SingleProp("--en-border-spacing-y"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin("border-spacing-y", SingleProp("--en-border-spacing-y"));
