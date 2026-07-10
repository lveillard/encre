#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

const fn plugin(namespace: &'static str, prop: PropertyName) -> (Plugin, Plugin) {
    (
        Plugin::new(PluginKind::Spacing { namespace, prop }).extra_lines(&[
            "border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);",
        ]),
        Plugin::new(PluginKind::Arbitrary { namespace, prop }).extra_lines(&[
            "border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);",
        ]),
    )
}

pub(crate) const PLUGIN: (Plugin, Plugin) = plugin(
    "border-spacing",
    MultipleProps(&["--en-border-spacing-x", "--en-border-spacing-y"]),
);
pub(crate) const PLUGIN_X: (Plugin, Plugin) =
    plugin("border-spacing-x", SingleProp("--en-border-spacing-x"));
pub(crate) const PLUGIN_Y: (Plugin, Plugin) =
    plugin("border-spacing-y", SingleProp("--en-border-spacing-y"));
