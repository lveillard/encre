#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::Spacing {
        prop,
        has_auto: false,
        has_full: false,
    }
    // TODO: extra_line: Some("border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);"),
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::OnlyArbitrary {
        prop,
        hints: &[],
        matcher: Length,
    }
    // TODO: extra_line: Some("border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);"),
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin(MultipleProps(&["--en-border-spacing-x", "--en-border-spacing-y"]));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin(MultipleProps(&["--en-border-spacing-x", "--en-border-spacing-y"]));

pub(crate) const PLUGIN_X_1: Plugin = builtin_plugin(SingleProp("--en-border-spacing-x"));
pub(crate) const PLUGIN_X_2: Plugin = arbitrary_plugin(SingleProp("--en-border-spacing-x"));

pub(crate) const PLUGIN_Y_1: Plugin = builtin_plugin(SingleProp("--en-border-spacing-y"));
pub(crate) const PLUGIN_Y_2: Plugin = arbitrary_plugin(SingleProp("--en-border-spacing-y"));
