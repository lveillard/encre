//! Define a plugin used to generate CSS properties quickly.
//!
//! Used for arbitrary CSS properties like `[mask-type:luminance]`.
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::Functional(Functional {
    namespace: "",        // field not used for css-property
    can_handle: |_| true, // field not used for css-property
    handle: |context| match context.modifier {
        Modifier::Builtin { .. } => unreachable!(),
        Modifier::Arbitrary { value, .. } => {
            generate_wrapper(context, |context| {
                for line in value.lines() {
                    if let Some((prop, value)) = line.split_once(':') {
                        context.buffer.line(format_args!("{prop}: {value};"));
                    }
                }
            });
        }
    },
    ..Functional::default()
});
