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

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: MultipleProps(&["-webkit-animation", "animation"]),
    values: map! {
        "animate-none" => "none",
        "animate-spin" => "spin 1s linear infinite",
        "animate-ping" => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite",
        "animate-pulse" => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "animate-bounce" => "bounce 1s infinite",
    },
    extra_css: Some(map! {
        "animate-none" => "",
        "animate-spin" => SPIN_ANIMATION,
        "animate-ping" => PING_ANIMATION,
        "animate-pulse" => PULSE_ANIMATION,
        "animate-bounce" => BOUNCE_ANIMATION,
    }),
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "animate",
    prop: MultipleProps(&["-webkit-animation", "animation"]),
    ..Arbitrary::default()
});
