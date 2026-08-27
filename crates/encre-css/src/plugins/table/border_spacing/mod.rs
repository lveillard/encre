#![doc = include_str!("README.md")]
#![doc(alias = "table")]
use crate::prelude::build_plugin::*;

const BORDER_SPACING_CSS: &str =
    "border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);";

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> (StaticPlugin, StaticPlugin) {
    (
        Plugin::Spacing(Spacing {
            namespace,
            prop,
            extra_rule_css: Some(&[BORDER_SPACING_CSS]),
            ..Spacing::default()
        }),
        Plugin::Arbitrary(Arbitrary {
            namespace,
            prop,
            extra_rule_css: Some(&[BORDER_SPACING_CSS]),
            ..Arbitrary::default()
        }),
    )
}

pub(crate) const PLUGIN: (StaticPlugin, StaticPlugin) = plugin(
    "border-spacing",
    MultipleProps(&["--en-border-spacing-x", "--en-border-spacing-y"]),
);
pub(crate) const PLUGIN_X: (StaticPlugin, StaticPlugin) =
    plugin("border-spacing-x", SingleProp("--en-border-spacing-x"));
pub(crate) const PLUGIN_Y: (StaticPlugin, StaticPlugin) =
    plugin("border-spacing-y", SingleProp("--en-border-spacing-y"));
