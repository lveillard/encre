Add support for an extra slash (`/`) followed by a value for the utility classes handled by this plugin.

In practice, you give it a map with the accepted values after the slash as keys and the values you
need to insert into the CSS value as values, as well as a default value in case no slash is found.
Then, you add the placeholder `{/}` to your CSS values (or templates) and it will be replaced with
the specified value.

### Example

```
use encre_css::{Config, generate};
use encre_css::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("background-image"),
    values: map! {
        "custom-bg-img" => "linear-gradient(to top in {/}, #00aaff, #00ffaa)",
    },
    extra_slash: Some(ExtraSlash {
        values: map! {
            "increasing" => "oklch increasing hue",
            "oklab" => "oklab",
        },
        default: "oklab",
    }),
    ..ListValues::default()
});

let mut config = Config::default();
config.register_plugin(&PLUGIN);

let generated = generate(["custom-bg-img", "custom-bg-img/increasing"], &config);

assert!(generated.ends_with(".custom-bg-img {
  background-image: linear-gradient(to top in oklab, #00aaff, #00ffaa);
}

.custom-bg-img\\/increasing {
  background-image: linear-gradient(to top in oklch increasing hue, #00aaff, #00ffaa);
}"));
```
