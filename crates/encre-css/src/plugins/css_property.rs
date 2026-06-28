//! Define a plugin used to generate CSS properties quickly.
//!
//! Used for arbitrary CSS properties like `[mask-type:luminance]`.
use crate::prelude::build_plugin::*;

// FIXME
pub(crate) const PLUGIN: Plugin = Plugin::ListCases { cases: phf_map! {} };

// #[derive(Debug)]
// pub(crate) struct CssPropertyPlugin;
//
// impl Plugin for CssPropertyPlugin {
//     fn can_handle(&self, _context: ContextCanHandle) -> bool {
//         // NOTE: No need to implement it because we are manually calling the `handle` method in `selector.rs`
//         unreachable!();
//     }
//
//     fn handle(&self, context: &mut ContextHandle) {
//         match context.modifier {
//             Modifier::Builtin { .. } => unreachable!(),
//             Modifier::Arbitrary { value, .. } => {
//                 for line in value.lines() {
//                     if let Some((prop, value)) = line.split_once(':') {
//                         context.buffer.line(format_args!("{prop}: {value};"));
//                     }
//                 }
//             }
//         }
//     }
// }
