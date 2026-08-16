Add one or several extra CSS line(s) **outside** the CSS rule generated for the utility class.

The argument is a map which allows choosing the added CSS based on the modifier value.

### Example

```
use encre_css::{Config, generate};
use encre_css::prelude::build_plugin::*;

const SPIN_ANIMATION: &str = "@keyframes anim-spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}\n\n";

const FADE_IN_ANIMATION: &str = "@keyframes anim-fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}\n\n";

const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("animation"),
    values: map! {
        "custom-animate-spin" => "anim-spin",
        "custom-animate-fade-in" => "anim-fade-in",
    },
    extra_css: Some(map! {
    "custom-animate-spin" => SPIN_ANIMATION,
    "custom-animate-fade-in" => FADE_IN_ANIMATION,
}),
    ..ListValues::default()
});

let mut config = Config::default();
config.register_plugin(&PLUGIN);

let generated = generate(["custom-animate-spin"], &config);

assert!(generated.ends_with(r"@keyframes anim-spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.custom-animate-spin {
  animation: anim-spin;
}"));
```
