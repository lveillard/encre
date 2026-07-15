#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::{plugins::transform::CSS_TRANSFORM, prelude::build_plugin::*};

type P = (StaticPlugin, StaticPlugin);

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> P {
    (
        Plugin::new(PluginKind::Number {
            namespace,
            prop,
            divide_by: 100.0,
        })
        .has_negative()
        .extra_lines(&[CSS_TRANSFORM]),
        Plugin::new(PluginKind::Arbitrary { namespace, prop }).extra_lines(&[CSS_TRANSFORM]),
    )
}

pub(crate) const PLUGIN: P = plugin("scale", MultipleProps(&["--en-scale-x", "--en-scale-y"]));
pub(crate) const PLUGIN_X: P = plugin("scale-x", SingleProp("--en-scale-x"));
pub(crate) const PLUGIN_Y: P = plugin("scale-y", SingleProp("--en-scale-y"));
pub(crate) const PLUGIN_Z: P = plugin("scale-z", SingleProp("--en-scale-z"));
