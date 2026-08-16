#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

type P = (StaticPlugin, StaticPlugin);

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> P {
    (
        Plugin::Spacing(Spacing {
            namespace,
            prop,
            ..Spacing::default()
        }),
        Plugin::Arbitrary(Arbitrary {
            namespace,
            prop,
            ..Arbitrary::default()
        }),
    )
}

pub(crate) const PLUGIN: P = plugin("scroll-m", SingleProp("scroll-margin"));
pub(crate) const PLUGIN_X: P = plugin("scroll-mx", SingleProp("scroll-margin-inline"));
pub(crate) const PLUGIN_Y: P = plugin("scroll-my", SingleProp("scroll-margin-block"));
pub(crate) const PLUGIN_START: P = plugin("scroll-ms", SingleProp("scroll-margin-inline-start"));
pub(crate) const PLUGIN_END: P = plugin("scroll-me", SingleProp("scroll-margin-inline-end"));
pub(crate) const PLUGIN_TOP: P = plugin("scroll-mt", SingleProp("scroll-margin-top"));
pub(crate) const PLUGIN_BOTTOM: P = plugin("scroll-mb", SingleProp("scroll-margin-bottom"));
pub(crate) const PLUGIN_LEFT: P = plugin("scroll-ml", SingleProp("scroll-margin-left"));
pub(crate) const PLUGIN_RIGHT: P = plugin("scroll-mr", SingleProp("scroll-margin-right"));
