//! Define a plugin used to generate CSS properties quickly.
//!
//! Used for arbitrary CSS properties like `[mask-type:luminance]`.
use crate::{generator::{generate_at_rules, generate_class}, prelude::build_plugin::*};

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Functional {
    namespace: "", // field not used for css-property
    handle: |context| match context.modifier {
        Modifier::Builtin { .. } => unreachable!(),
        Modifier::Arbitrary { value, .. } => {
            generate_at_rules(context, |context| {
                generate_class(
                    context,
                    |context| {
                        for line in value.lines() {
                            if let Some((prop, value)) = line.split_once(':') {
                                context.buffer.line(format_args!("{prop}: {value};"));
                            }
                        }
                    },
                    "",
                );
            });
        }
    },
});
