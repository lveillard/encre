#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

const SPIN_ANIMATION: &str = "@-webkit-keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}\n\n";

const PING_ANIMATION: &str = "@-webkit-keyframes ping {
  75%, 100% {
    transform: scale(2);
    opacity: 0;
  }
}

@keyframes ping {
  75%, 100% {
    transform: scale(2);
    opacity: 0;
  }
}\n\n";

const PULSE_ANIMATION: &str = "@-webkit-keyframes pulse {
  50% {
    opacity: .5;
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: .5;
  }
}\n\n";

const BOUNCE_ANIMATION: &str = "@-webkit-keyframes bounce {
  0%, 100% {
    transform: translateY(-25%);
    -webkit-animation-timing-function: cubic-bezier(0.8,0,1,1);
    animation-timing-function: cubic-bezier(0.8,0,1,1);
  }

  50% {
    transform: none;
    -webkit-animation-timing-function: cubic-bezier(0,0,0.2,1);
    animation-timing-function: cubic-bezier(0,0,0.2,1);
  }
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(-25%);
    -webkit-animation-timing-function: cubic-bezier(0.8,0,1,1);
    animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
  }
  50% {
    transform: translateY(0);
    -webkit-animation-timing-function: cubic-bezier(0,0,0.2,1);
    animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
  }
}\n\n";

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn needs_wrapping(&self) -> bool {
        false
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["spin", "ping", "pulse", "bounce", "none"].contains(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let animation = match *value {
                    "none" => "none",
                    "spin" => {
                        context.buffer.raw(SPIN_ANIMATION);
                        "spin 1s linear infinite"
                    }
                    "ping" => {
                        context.buffer.raw(PING_ANIMATION);
                        "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite"
                    }
                    "pulse" => {
                        context.buffer.raw(PULSE_ANIMATION);
                        "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"
                    }
                    "bounce" => {
                        context.buffer.raw(BOUNCE_ANIMATION);
                        "bounce 1s infinite"
                    }
                    _ => unreachable!(),
                });

                generate_wrapper(context, |context| {
                    context.buffer.lines([
                        format_args!("-webkit-animation: {animation});"),
                        format_args!("animation: {animation});"),
                    ]);
                });
            }
            Modifier::Arbitrary { value, .. } => generate_wrapper(context, |context| {
                context.buffer.lines([
                    format_args!("-webkit-animation: {value});"),
                    format_args!("animation: {value});"),
                ]);
            }),
        }
    }
}
