#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

type P = (StaticPlugin, StaticPlugin);

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> P {
    (
        Plugin::new(PluginKind::Spacing {
            namespace,
            prop,
        }),
        Plugin::new(PluginKind::Arbitrary { namespace, prop }),
    )
}

pub(crate) const PLUGIN: P = plugin("scroll-p", SingleProp("scroll-padding"));
pub(crate) const PLUGIN_X: P = plugin("scroll-px", SingleProp("scroll-padding-inline"));
pub(crate) const PLUGIN_Y: P = plugin("scroll-py", SingleProp("scroll-padding-block"));
pub(crate) const PLUGIN_START: P = plugin("scroll-ps", SingleProp("scroll-padding-inline-start"));
pub(crate) const PLUGIN_END: P = plugin("scroll-pe", SingleProp("scroll-padding-inline-end"));
pub(crate) const PLUGIN_TOP: P = plugin("scroll-pt", SingleProp("scroll-padding-top"));
pub(crate) const PLUGIN_BOTTOM: P = plugin("scroll-pb", SingleProp("scroll-padding-bottom"));
pub(crate) const PLUGIN_LEFT: P = plugin("scroll-pl", SingleProp("scroll-padding-left"));
pub(crate) const PLUGIN_RIGHT: P = plugin("scroll-pr", SingleProp("scroll-padding-right"));
