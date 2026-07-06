#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

type P = (Plugin, Plugin);

const fn plugin(prefix: &'static str, prop: PropertyName) -> P {
    (
        Plugin::new(PluginKind::Spacing {
            prefix,
            prop,
            has_auto: false,
            has_full: false,
        }),
        Plugin::new(PluginKind::Arbitrary { prefix, prop }),
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
