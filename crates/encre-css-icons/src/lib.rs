//! A plugin that provides a set of classes to use icons from [Iconify](https://iconify.design)
//! based on the [`@unocss/preset-icons`](https://github.com/unocss/unocss/tree/main/packages/preset-icons)
//! NodeJS package.
//!
//! ### Features
//!
//! - A **lot** of icon sets (you can use [Icônes](https://icones.netlify.app) to search
//!   among them)
//! - Pure CSS icons
//! - Icons follow the text size
//! - Icons follow the text color
//! - Support colorful and monochrome icons
//! - No request is issued client-side when not using WebAssembly
//! - Supports WebAssembly (SVG icons will be directly fetched client-side)
//!
//! ### Getting started
//!
//! To integrate `encre-css-icons` with `encre-css`, add it in your `Cargo.toml`:
//!
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[dependencies]</span>
//! encre-css-icons = <span class="string">"0.7.0"</span></code></pre></div>
//!
//! Then, call the `register` function with a mutable reference to a `Config`
//! structure and some parameters (all parameters are optional, you can use `None`
//! to use the default value):
//!
//! ```no_run
//! use encre_css::Config;
//!
//! # fn main() -> encre_css::Result<()> {
//! let mut config = Config::from_file("encre-css.toml")?;
//! // Or let mut config = Config::default();
//!
//! encre_css_icons::register(&mut config);
//!
//! // The convention is <prefix><collection>-<icon>
//! let _generated = encre_css::generate(
//!     [r#"<h1 class="text-xl text-gray-600">Hello <span class="subway-world-1"></span>!</h1><div class="mdi-alarm block"></div><span class="fa-solid-home"></span><span class="openmoji-automobile hover:openmoji-autonomous-car"></span>"#],
//!     &config,
//! );
//!
//! # Ok(())
//! # }
//! // Do something with the CSS
//! ```
//!
//! ### Configuration
//!
//! This plugin has some configuration options, to set them simply add an extra field `icons` in
//! the configuration with the fields you want to change. If configuring using Rust, the extra
//! configuration must be added **before** registering the plugin. For example in TOML:
//!
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[extra.icons]</span>
//! prefix = <span class="string">"i-"</span>
//! custom-cdn = <span class="string">"https://cdn.skypack.dev"</span>
//! scale = <span class="number">1.2</span></code></pre></div>
//!
//! Or in Rust:
//!
//! ```
//! use encre_css::{Config, toml};
//!
//! let mut config = Config::default();
//! // Or let mut config = Config::from_file("encre-css.toml")?;
//!
//! config.extra.add("icons", toml! {
//!     prefix = "i-"
//!     custom-cdn = "https://cdn.skypack.dev"
//!     scale = 1.2
//! });
//!
//! // Then register the plugin
//! encre_css_icons::register(&mut config);
//! ```
//!
//! Configuration fields:
//!
//! - `prefix` (default: `""`): a static prefix added to all icons
//! - `custom-cdn` (default: `"https://esm.sh"`): the CDN used to get the SVG definition of icons (see the section below)
//! - `scale` (default: `1.0`): the scale of icons used to change their size
//!
//! ### Network requests and caching
//!
//! Please note that, in order to get the SVG definition of icons, this crate will make
//! requests to a (of course configurable) third-party CDN (the default CDN is `https://esm.sh`)
//! and will cache them in the system's configured cache directory (`$XDG_CACHE_HOME`
//! or `$HOME/.cache` on GNU/Linux, `{FOLDERID_LocalAppData}` on Windows, `$HOME/Library/Caches`
//! on macOS), in a directory named `encre-css-icons-cache`.
//!
//! ### Various tips and tricks
//!
//! If searching the collection files takes too long (even if they are also cached
//! in memory), it is recommended to optimize the `encre-css-icons` crate even in
//! development, by adding the following in your `Cargo.toml` file:
//!
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[profile.dev.package.encre-css-icons]</span>
//! opt-level = <span class="number">3</span></code></pre></div>
//!
//! By default, each icon has the `display: inline-block;` CSS property applied
//! (even on `div` elements) otherwise they would be unsized when used in `span`
//! elements. If you need to turn them into block elements, you can use the `block`
//! utility class on each icon, e.g. `<div class="fa-pencil block"></div>`.
//!
//! ### Cargo features
//!
//! - `fs-cache` (enabled by default): store all fetched icons in the cache directory of the system
//!   to avoid requesting them each time from the CDN
//! - `embed-icons`: embed all the JSON files of icons in a directory into the binary to avoid
//!   requesting them online. The `register` function takes a second argument of type
//!   `include_dir::Dir` when this feature is enabled
//!
//! ### License
//!
//! The code itself is under the [MIT License](../../LICENSE).
//! The collections of icons are under various licenses, see
//! [collections.md](https://github.com/iconify/icon-sets/blob/master/collections.md)
//! for a list of collections and their licenses.

use encre_css::{prelude::build_plugin::*, selector::Modifier, Config};

#[cfg(all(
    feature = "fs-cache",
    not(feature = "embed-icons"),
    not(target_arch = "wasm32")
))]
use directories::BaseDirs;

#[cfg(not(target_arch = "wasm32"))]
use once_cell::sync::Lazy;

#[cfg(feature = "embed-icons")]
use std::sync::OnceLock;
use std::{collections::BTreeMap, sync::Mutex};

#[cfg(all(
    feature = "fs-cache",
    not(feature = "embed-icons"),
    not(target_arch = "wasm32")
))]
use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use std::borrow::Cow;

#[cfg(not(target_arch = "wasm32"))]
pub mod collection;

#[cfg(not(target_arch = "wasm32"))]
use collection::{Collection, IconOptional};

const COLLECTIONS: &[&str] = &[
    "material-symbols",
    "ic",
    "mdi",
    "ph",
    "ri",
    "carbon",
    "bi",
    "tabler",
    "ion",
    "uil",
    "teenyicons",
    "clarity",
    "iconoir",
    "majesticons",
    "zondicons",
    "ant-design",
    "bx",
    "bxs",
    "gg",
    "cil",
    "lucide",
    "pixelarticons",
    "system-uicons",
    "ci",
    "akar-icons",
    "typcn",
    "radix-icons",
    "ep",
    "mdi-light",
    "fe",
    "eos-icons",
    "line-md",
    "charm",
    "prime",
    "heroicons-outline",
    "heroicons-solid",
    "uiw",
    "uim",
    "uit",
    "uis",
    "maki",
    "gridicons",
    "mi",
    "quill",
    "gala",
    "fluent",
    "icon-park-outline",
    "icon-park",
    "vscode-icons",
    "jam",
    "codicon",
    "pepicons",
    "bytesize",
    "ei",
    "fa6-solid",
    "fa6-regular",
    "octicon",
    "ooui",
    "nimbus",
    "openmoji",
    "twemoji",
    "noto",
    "noto-v1",
    "emojione",
    "emojione-monotone",
    "emojione-v1",
    "fxemoji",
    "bxl",
    "logos",
    "simple-icons",
    "cib",
    "fa6-brands",
    "arcticons",
    "file-icons",
    "brandico",
    "entypo-social",
    "cryptocurrency",
    "flag",
    "circle-flags",
    "flagpack",
    "cif",
    "gis",
    "map",
    "geo",
    "fad",
    "academicons",
    "wi",
    "healthicons",
    "medical-icon",
    "la",
    "eva",
    "dashicons",
    "flat-color-icons",
    "entypo",
    "foundation",
    "raphael",
    "icons8",
    "iwwa",
    "fa-solid",
    "fa-regular",
    "fa-brands",
    "fa",
    "fontisto",
    "icomoon-free",
    "ps",
    "subway",
    "oi",
    "wpf",
    "simple-line-icons",
    "et",
    "el",
    "vaadin",
    "grommet-icons",
    "whh",
    "si-glyph",
    "zmdi",
    "ls",
    "bpmn",
    "flat-ui",
    "vs",
    "topcoat",
    "il",
    "websymbol",
    "fontelico",
    "feather",
    "mono-icons",
];

#[cfg(not(any(target_arch = "wasm32", feature = "embed-icons")))]
const DEFAULT_CDN: &str = "https://esm.sh";

#[cfg(all(
    feature = "fs-cache",
    not(feature = "embed-icons"),
    not(target_arch = "wasm32")
))]
const CACHE_SUB_DIR_NAME: &str = "encre-css-icons-cache";

#[cfg(not(target_arch = "wasm32"))]
static MEM_CACHE: Lazy<Mutex<BTreeMap<&'static str, Collection>>> =
    Lazy::new(|| Mutex::new(BTreeMap::new()));

#[cfg(feature = "embed-icons")]
static EMBEDDED_ICONS: OnceLock<include_dir::Dir> = OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
fn get_icon(
    config: &Config,
    collection: &Collection,
    icon_name: &str,
) -> Option<((String, String), String)> {
    let icon = if let Some(icon) = collection.icons.get(icon_name) {
        Cow::Borrowed(icon)
    } else if let Some(alias) = collection.aliases.get(icon_name) {
        if let Some(icon) = collection.icons.get(&alias.parent) {
            // Merge optional properties following the logic described in
            // https://docs.iconify.design/types/iconify-json.html
            let mut icon = icon.clone();
            icon.optional.top = alias.optional.top;
            icon.optional.left = alias.optional.left;
            icon.optional.width = alias.optional.width;
            icon.optional.height = alias.optional.height;
            icon.optional.rotate = (icon.optional.rotate + alias.optional.rotate) % 4.;
            icon.optional.h_flip = alias.optional.h_flip != icon.optional.h_flip;
            icon.optional.v_flip = alias.optional.v_flip != icon.optional.v_flip;
            Cow::Owned(icon)
        } else {
            return None;
        }
    } else if let Some(character) = collection.chars.get(icon_name) {
        if let Some(icon) = collection.icons.get(character) {
            Cow::Borrowed(icon)
        } else {
            return None;
        }
    } else {
        return None;
    };

    let IconOptional {
        mut left,
        mut top,
        h_flip,
        v_flip,
        ..
    } = icon.optional;
    let mut width = icon.optional.width.unwrap_or(collection.width);
    let mut height = icon.optional.height.unwrap_or(collection.height);
    let mut rotation = icon.optional.rotate;

    // The icon is flipped first, then rotated
    let after_transforms = if h_flip {
        if v_flip {
            rotation += 2.;
            String::new()
        } else {
            // Horizontal flip
            left = 0.;
            top = 0.;
            format!(
                "translate({} {}) scale(-1 1)",
                width + left,
                -(top as isize)
            )
        }
    } else if v_flip {
        left = 0.;
        top = 0.;
        format!(
            "translate({} {}) scale(1 -1)",
            -(left as isize),
            height + top
        )
    } else {
        String::new()
    };

    if rotation < 0. {
        rotation -= f32::floor(rotation / 4.) * 4.;
    }

    let rotation = (rotation % 4.) as usize;

    let before_transforms = match rotation {
        // 90 deg
        1 => format!("rotate(90 {val} {val})", val = height / 2. + top),

        // 180 deg
        2 => format!("rotate(180 {} {})", width / 2. + left, height / 2. + top),

        // 270 deg
        3 => format!("rotate(270 {val} {val})", val = width / 2. + left),

        _ => String::new(),
    };

    if rotation % 2 == 1 {
        // Swap width/height and top/left for 90deg or 270deg rotation
        if left != 0. || top != 0. {
            (left, top) = (top, left);
        }

        if width != height {
            (width, height) = (height, width);
        }
    }

    let scale = if let Some(icons_config) = config.extra.get("icons") {
        if let Some(table) = icons_config.as_table() {
            if let Some(scale_value) = table.get("scale") {
                if let Some(scale) = scale_value.as_float() {
                    scale
                } else {
                    println!("Bad type for the `scale` extra field (in the `icons` field): expected `Float`, found `{}`. Using the default scale instead.", scale_value.type_str());
                    1.
                }
            } else {
                1.
            }
        } else {
            println!("Bad type for the `icons` extra field: expected `Table`, found `{}`. Using the default scale instead.", icons_config.type_str());
            1.
        }
    } else {
        1.
    };
    let formatted_width = format!("{}em", (width / height) * scale as f32);
    let formatted_height = format!("{scale}em");

    let svg = {
        let body = if !before_transforms.is_empty() || !after_transforms.is_empty() {
            Cow::Owned(format!(
                r#"<g transform="{}{}{}">{}</g>"#,
                before_transforms,
                if before_transforms.is_empty() {
                    ""
                } else {
                    " "
                },
                after_transforms,
                icon.body
            ))
        } else {
            Cow::Borrowed(&icon.body)
        };

        // Optimize the SVG (from https://bl.ocks.org/jennyknuth/222825e315d45a738ed9d6e04c7a88d0)
        // And create a Data URI containing its data
        format!(r#"data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{formatted_width}" height="{formatted_height}" preserveAspectRatio="XMidYMid meet" viewBox="{left} {top} {width} {height}">{body}</svg>"#)
            .replace('"', "\'")
            .replace('%', "%25")
            .replace('#', "%23")
            .replace('{', "%7B")
            .replace('}', "%7D")
            .replace('<', "%3C")
            .replace('>', "%3E")
    };

    Some(((formatted_width, formatted_height), svg))
}

#[cfg(not(any(target_arch = "wasm32", feature = "embed-icons")))]
fn fetch_or_cache_collection(config: &Config, collection: &'static str) {
    if MEM_CACHE.lock().unwrap().get(collection).is_some() {
        // Collection already in the memory cache, use it
        return;
    }

    #[cfg(feature = "fs-cache")]
    let cache_dir = match BaseDirs::new() {
        Some(dirs) => PathBuf::from(dirs.cache_dir()),
        None => {
            // If the home directory is not set, use the current directory
            env::current_dir().unwrap_or(std::path::PathBuf::from("."))
        }
    };

    #[cfg(feature = "fs-cache")]
    let collection_file = cache_dir
        .join(CACHE_SUB_DIR_NAME)
        .join(collection)
        .with_extension("json");

    #[cfg(feature = "fs-cache")]
    if collection_file.exists() {
        // File already in cache, use it
        if let Some(content) = File::open(&collection_file)
            .ok()
            .map(|f| BufReader::new(f))
            .and_then(|reader| serde_json::from_reader(reader).ok())
        {
            MEM_CACHE.lock().unwrap().insert(collection, content);
        }
        return;
    }

    // Fetch the file
    let custom_cdn = if let Some(icons_config) = config.extra.get("icons") {
        if let Some(table) = icons_config.as_table() {
            if let Some(cdn_value) = table.get("custom-cdn") {
                if let Some(cdn) = cdn_value.as_str() {
                    Cow::Owned(cdn.trim_end_matches('/').to_string())
                } else {
                    println!("Bad type for the `custom-cdn` extra field (in the `icons` field): expected `String`, found `{}`. Using the default CDN instead.", cdn_value.type_str());
                    Cow::Borrowed(DEFAULT_CDN)
                }
            } else {
                Cow::Borrowed(DEFAULT_CDN)
            }
        } else {
            println!("Bad type for the `icons` extra field: expected `Table`, found `{}`. Using the default CDN instead.", icons_config.type_str());
            Cow::Borrowed(DEFAULT_CDN)
        }
    } else {
        Cow::Borrowed(DEFAULT_CDN)
    };

    let url = format!("{custom_cdn}/@iconify-json/{collection}/icons.json");

    let mut res = match ureq::get(&url).call() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("encre_css_icons: failed to download the icon pack: {e}");
            return;
        }
    };

    let content = match res.body_mut().read_to_string() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("encre_css_icons: failed to read the content of the icon pack: {e}");
            return;
        }
    };

    let json = match serde_json::from_str(&content) {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "encre_css_icons: failed to deserialize the content of the icon pack to JSON: {e}"
            );
            return;
        }
    };

    #[cfg(feature = "fs-cache")]
    {
        // Write the cached file to the disk
        if let Err(e) = fs::create_dir_all(collection_file.with_file_name("")) {
            eprintln!("encre_css_icons: failed to write the cached file to the disk: {e}");
        }

        if let Err(e) = fs::write(collection_file, &content) {
            eprintln!("encre_css_icons: failed to write the cached file to the disk: {e}");
        }
    }

    MEM_CACHE.lock().unwrap().insert(collection, json);
}

const PLUGIN: Plugin = Plugin::new(PluginKind::Functional {
    namespace: "",
    can_handle: |context| {
        let Modifier::Builtin { value, .. } = context.modifier else {
            return false;
        };

        let prefix = if let Some(icons_config) = context.config.extra.get("icons") {
            if let Some(table) = icons_config.as_table() {
                if let Some(prefix_value) = table.get("prefix") {
                    if let Some(prefix) = prefix_value.as_str() {
                        Cow::Owned(prefix.trim_end_matches('-').to_string())
                    } else {
                        println!("Bad type for the `prefix` extra field (in the `icons` field): expected `String`, found `{}`. Using the default prefix instead.", prefix_value.type_str());
                        Cow::Borrowed("")
                    }
                } else {
                    Cow::Borrowed("")
                }
            } else {
                println!("Bad type for the `icons` extra field: expected `Table`, found `{}`. Using the default prefix instead.", icons_config.type_str());
                Cow::Borrowed("")
            }
        } else {
            Cow::Borrowed("")
        };

        let Some(mut value) = value.strip_prefix(&*prefix) else {
            return false;
        };

        if value.starts_with('-') {
            value = &value[1..];
        }

        COLLECTIONS.iter().any(|c| value.starts_with(c))
    },
    handle: |context| {
        let Modifier::Builtin { value, .. } = context.modifier else {
            unreachable!("checked by can_handle");
        };

        let prefix = if let Some(icons_config) = context.config.extra.get("icons") {
            if let Some(table) = icons_config.as_table() {
                if let Some(prefix_value) = table.get("prefix") {
                    if let Some(prefix) = prefix_value.as_str() {
                        Cow::Owned(prefix.trim_end_matches('-').to_string())
                    } else {
                        println!("Bad type for the `prefix` extra field (in the `icons` field): expected `String`, found `{}`. Using the default prefix instead.", prefix_value.type_str());
                        Cow::Borrowed("")
                    }
                } else {
                    Cow::Borrowed("")
                }
            } else {
                println!("Bad type for the `icons` extra field: expected `Table`, found `{}`. Using the default prefix instead.", icons_config.type_str());
                Cow::Borrowed("")
            }
        } else {
            Cow::Borrowed("")
        };

        let Some(mut value) = value.strip_prefix(&*prefix) else {
            return;
        };

        if value.starts_with('-') {
            value = &value[1..];
        }

        // Unwrapping won't panic because it's asserted by the `can_handle` method
        let (collection, rest) = COLLECTIONS
            .iter()
            .find_map(|c| value.strip_prefix(c).map(|r| (c, r)))
            .unwrap();

        let icon = rest.strip_prefix('-').unwrap_or(rest);

        #[cfg(not(any(target_arch = "wasm32", feature = "embed-icons")))]
        fetch_or_cache_collection(context.config, collection);

        #[cfg(target_arch = "wasm32")]
        {
            generate_wrapper(context, |context| {
                context.buffer.lines([
                    format_args!(
                        r#"--en-icon: url("https://api.iconify.design/{collection}/{icon}.svg");"#
                    ),
                    format_args!("mask: var(--en-icon) no-repeat;"),
                    format_args!("mask-size: 100% 100%;"),
                    format_args!("-webkit-mask: var(--en-icon) no-repeat;"),
                    format_args!("-webkit-mask-size: 100% 100%;"),
                    format_args!("background-color: currentColor;"),
                    format_args!("display: inline-block;"),
                    format_args!("width: 32px;"),
                    format_args!("height: 32px;"),
                ]);
            });
        }

        #[cfg(feature = "embed-icons")]
        if let Some(json) = EMBEDDED_ICONS
            .get()
            .and_then(|i| i.get_file(format!("{collection}.json")))
            .and_then(|f| f.contents_utf8())
            .and_then(|c| serde_json::from_str(&c).ok())
        {
            MEM_CACHE.lock().unwrap().insert(collection, json);
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(coll) = MEM_CACHE.lock().unwrap().get(collection) {
            if let Some(((width, height), icon_data_uri)) = get_icon(context.config, coll, icon) {
                generate_wrapper(context, |context| {
                    if icon_data_uri.contains("currentColor") {
                        // From https://codepen.io/noahblon/post/coloring-svgs-in-css-background-images
                        context.buffer.lines([
                            format_args!(r#"--en-icon: url("{icon_data_uri}");"#),
                            format_args!("mask: var(--en-icon) no-repeat;"),
                            format_args!("mask-size: 100% 100%;"),
                            format_args!("-webkit-mask: var(--en-icon) no-repeat;"),
                            format_args!("-webkit-mask-size: 100% 100%;"),
                            format_args!("background-color: currentColor;"),
                        ]);
                    } else {
                        context.buffer.lines([
                            format_args!(r#"background: url("{icon_data_uri}") no-repeat center;"#),
                            format_args!("background-color: transparent;"),
                            format_args!("background-size: 100% 100%;"),
                        ]);
                    }

                    context.buffer.lines([
                        format_args!("display: inline-block;"),
                        format_args!("width: {width};"),
                        format_args!("height: {height};"),
                    ]);
                });
            }
        } else {
            eprintln!("encre_css_icons: Warning: the collection `{collection}` is not loaded but referenced. It can happen if you embed icons in the binary and forgot to add the JSON file containing the icons in the directory you specified");
        }
    },
});

#[cfg(not(feature = "embed-icons"))]
pub fn register(config: &mut Config) {
    config.register_plugin(&PLUGIN);
}

#[cfg(feature = "embed-icons")]
pub fn register(config: &mut Config, embedded_dir: include_dir::Dir<'static>) {
    // When EMBEDDED_ICONS is already set, just do nothing
    let _ = EMBEDDED_ICONS.set(embedded_dir);
    config.register_plugin(&PLUGIN);
}

#[cfg(test)]
mod tests {
    use encre_css::{toml, Config, Preflight};

    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn simple() {
        let content = r#"<div class="fa-solid-plus"><>"#;
        let mut config = Config::default();
        config.preflight = Preflight::None;
        super::register(&mut config);

        let generated = encre_css::generate([content], &config);
        assert!(generated.contains(".fa-solid-plus"));
    }

    #[test]
    fn with_prefix() {
        let content = r#"<div class="i-fa-solid-plus fa-solid-home"><>"#;
        let mut config = Config::default();
        config.preflight = Preflight::None;
        config.extra.add(
            "icons",
            toml! {
                prefix = "i-"
            },
        );
        super::register(&mut config);

        let generated = encre_css::generate([content], &config);
        dbg!(&generated);
        assert!(generated.contains(".i-fa-solid-plus"));
        assert!(!generated.contains(".fa-solid-home"));
    }

    #[test]
    fn full_test_fixture() {
        let content = fs::read_to_string("tests/fixtures/icons.html").unwrap();
        let expected = fs::read_to_string("tests/fixtures/icons.css").unwrap();

        let mut config = Config::default();
        config.extra.add(
            "icons",
            toml! {
                prefix = "i-"
            },
        );
        super::register(&mut config);

        let generated = encre_css::generate([content.as_str()], &config);
        assert_eq!(generated, expected.trim_end());
    }
}
