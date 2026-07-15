//! A TailwindCSS-compatible utility-first CSS generation library written in Rust.
//!
//! ## A brief introduction to utility-first CSS frameworks
//!
//! Traditionally, whenever you need to style something on the web, you write CSS in a dedicated
//! file and apply the rules using classes in your HTML, like that:
//!
//! <div style="margin-top: 1rem; margin-bottom: 1rem;"></div>
//!
//! <div class="notification">
//!   <div class="notification-header">
//!     <div class="app-icon"></div>
//!     A new Javascript library has been released!
//!   </div>
//!   <div class="notification-body">
//!     The library <code>react</code> has just been released, did you know it?
//!     It is <i>a JavaScript library for creating user interfaces</i>.
//!   </div>
//!   <div class="notification-footer">
//!     <a href="#" class="dismiss-button">Dismiss</a>
//!     <a href="#" class="try-button">Try it here!</a>
//!   </div>
//! </div>
//!
//! <div style="margin-top: 1rem; margin-bottom: 1rem;"></div>
//!
//! <style>
//! .notification {
//!   width: 30rem;
//!   box-shadow: 1px 1px 5px 2px #e5e7eb;
//!   border-radius: 0.8rem;
//!   font-family: sans-serif;
//! }
//!
//! .notification-header {
//!   padding: 0.5rem 1rem;
//!   display: flex;
//!   align-items: center;
//! }
//!
//! .app-icon {
//!   background-color: rgb(37 99 235);
//!   border-radius: 50%;
//!   width: 1.2rem;
//!   height: 1.2rem;
//!   margin-right: 1rem;
//! }
//!
//! .notification-body {
//!   padding: 1rem 1.5rem 1.5rem 1.5rem;
//! }
//!
//! .notification-footer {
//!   display: flex;
//!   justify-content: space-between;
//! }
//!
//! .dismiss-button {
//!   color: rgb(225 29 72);
//!   padding: 0.5rem 1rem;
//! }
//!
//! .try-button {
//!   background-color: rgb(37 99 235);
//!   color: white;
//!   border-bottom-right-radius: 0.8rem;
//!   border-top-left-radius: 0.8rem;
//!   padding: 0.5rem 1rem;
//!   box-shadow: 1px 1px 5px 1px rgb(37 99 235);
//! }
//! </style>
//
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">&lt;div</span> class=<span class="string">"notification"</span><span class="kw">&gt;
//!   &lt;div</span> class=<span class="string">"notification-header"</span><span class="kw">&gt;</span>
//!     <span class="kw">&lt;div</span> class=<span class="string">"app-icon"</span><span class="kw">&gt;&lt;/div&gt;</span>
//!     A new Javascript library has been released!
//!   <span class="kw">&lt;/div&gt;</span>
//!   <span class="kw">&lt;div</span> class=<span class="string">"notification-body"</span><span class="kw">&gt;</span>
//!     The library <span class="kw">&lt;code&gt;</span>react<span class="kw">&lt;/code&gt;</span> has just been released, did you know it?
//!     It is <span class="kw">&lt;i&gt;</span>a JavaScript library for creating user interfaces<span class="kw">&lt;/i&gt;</span>.
//!   <span class="kw">&lt;/div&gt;</span>
//!   <span class="kw">&lt;div</span> class=<span class="string">"notification-footer"</span><span class="kw">&gt;</span>
//!     <span class="kw">&lt;a</span> href=<span class="string">"#"</span> class=<span class="string">"dismiss-button"</span><span class="kw">&gt;</span>Dismiss<span class="kw">&lt;/a&gt;</span>
//!     <span class="kw">&lt;a</span> href=<span class="string">"#"</span> class=<span class="string">"try-button"</span>&gt;Try it here!<span class="kw">&lt;/a&gt;</span>
//!   <span class="kw">&lt;/div&gt;</span>
//! <span class="kw">&lt;/div&gt;</span>
//! </code></pre></div>
//!
//! However styling this way is pretty boring because you need to think of good class names and
//! you have to repeatedly switch between several files, it could be better. Utility-first CSS
//! frameworks takes a new approach by using minimal and pre-defined class names directly linked to
//! its CSS rule content. The CSS file will then be generated
//! [on-demand](https://antfu.me/posts/reimagine-atomic-css#on-demand-way) allowing the classes
//! to be very flexible and customizable. This approach lets you quickly prototype visual HTML
//! elements and encourages you to turn them later into components using your favorite web framework.
//! It also makes building a responsive website easier and forces it to be closer to your design
//! system:
//
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">&lt;div</span> class=<span class="string">"w-128 text-md shadow-[1px_1px_10px_2px_#e5e7eb] rounded-xl"</span><span class="kw">&gt;
//!   &lt;div</span> class=<span class="string">"p-3 flex items-center"</span><span class="kw">&gt;</span>
//!     <span class="kw">&lt;div</span> class=<span class="string">"bg-blue-500 rounded-full w-5 h-5 mr-3"</span><span class="kw">&gt;&lt;/div&gt;</span>
//!     A new Javascript library has been released!
//!   <span class="kw">&lt;/div&gt;</span>
//!   <span class="kw">&lt;div</span> class=<span class="string">"p-6 pt-4"</span><span class="kw">&gt;</span>
//!     The library <span class="kw">&lt;code&gt;</span>react<span class="kw">&lt;/code&gt;</span> has just been released, did you know it?
//!     It is <span class="kw">&lt;i&gt;</span>a JavaScript library for creating user interfaces<span class="kw">&lt;/i&gt;</span>.
//!   <span class="kw">&lt;/div&gt;</span>
//!   <span class="kw">&lt;div</span> class=<span class="string">"flex justify-between"</span><span class="kw">&gt;</span>
//!     <span class="kw">&lt;a</span> href=<span class="string">"#"</span> class=<span class="string">"p-3 text-rose-600"</span><span class="kw">&gt;</span>Dismiss<span class="kw">&lt;/a&gt;</span>
//!     <span class="kw">&lt;a</span> href=<span class="string">"#"</span> class=<span class="string">"p-3 bg-blue-600 text-white rounded-br-xl rounded-tl-xl shadow shadow-blue-600"</span>&gt;Try it here!<span class="kw">&lt;/a&gt;</span>
//!   <span class="kw">&lt;/div&gt;</span>
//! <span class="kw">&lt;/div&gt;</span>
//! </code></pre></div>
//!
//! There is already a lot of utility-first frameworks like [Tailwind
//! CSS](https://tailwindcss.com), [Windi CSS](https://windicss.org), [Twind](https://twind.dev)
//! and [Uno CSS](https://uno.antfu.me), but `encre-css` is unique because it's written in Rust and
//! uses a new fully-declarative architecture, making it **the fastest utility-first framework**
//! (according to the benchmark [here](https://gitlab.com/encre-org/encre-css-bench) based on
//! [Uno CSS' benchmark](https://github.com/unocss/unocss/tree/main/bench)). It's also very
//! [customizable](crate::plugins).
//!
//! ## Getting started
//!
//! Add `encre-css` to your `Cargo.toml`:
//!
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[dependencies]</span>
//! encre-css = <span class="string">"0.20.1"</span></code></pre></div>
//!
//! Generating styles takes two steps:
//! - You need to _configure_ the CSS generation by making a [`Config`] structure.
//!   It can be created by reading a [TOML](https://toml.io) file using
//!   [`Config::from_file`] or by using the default values with [`Config::default`];
//! - Then, you need to _generate the styles_ based on some sources using the [`generate`]
//!   function. This function will scan the content of the sources, extract atomic classes and
//!   generate the style needed for each class.
//!
//! ### Example
//!
//! ```
//! use encre_css::Config;
//!
//! let config = Config::default();
//! let generated = encre_css::generate(
//!     [r#"<p class="w-auto bg-red-200 rounded-md">Hello world!</p>"#],
//!     &config,
//! );
//!
//! assert!(generated.ends_with(".w-auto {
//!   width: auto;
//! }
//!
//! .rounded-md {
//!   border-radius: 0.375rem;
//! }
//!
//! .bg-red-200 {
//!   background-color: oklch(88.5% .062 18.334);
//! }"));
//! ```
//!
//! ### What to do next
//!
//! 1. The documentation about the composition of a class (also named selector) is [here](crate::selector).
//! 2. The documentation about all utility classes (handled by plugins) is [here](crate::plugins).
//! 3. The documentation about the reset CSS commonly used (also named preflight) is [here](crate::preflight).
//!
//! ## Command line interface
//!
//! A command line interface is also available. Install it using:
//!
//! ```bash
//! cargo install encre-css-cli
//! ```
//!
//! Then run `encrecss --help` for instructions on how to use it.
#![doc(html_logo_url = "https://gitlab.com/encre-org/encre-css/raw/main/.assets/logo.png")]
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unreachable_pub,
    rustdoc::private_doc_tests,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    clippy::unnecessary_wraps,
    clippy::too_many_lines,
    clippy::implicit_clone,
    clippy::explicit_iter_loop,
    clippy::unnecessary_cast,
    clippy::missing_errors_doc,
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::non_ascii_literal,
    clippy::dbg_macro,
    clippy::use_debug,
    clippy::map_err_ignore,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo
)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate, clippy::enum_glob_use, clippy::wildcard_imports)]

pub mod config;
pub mod error;
pub mod generator;
pub mod plugins;
pub mod preflight;
pub mod scanner;
pub mod selector;
pub mod utils;

#[doc(inline)]
pub use config::Config;

#[doc(inline)]
pub use error::{Error, Result};

#[doc(inline)]
pub use generator::generate;

#[doc(inline)]
pub use preflight::Preflight;

#[doc(inline)]
pub use scanner::Scanner;

pub use toml::toml;

/// Various helper preludes.
pub mod prelude {
    /// Prelude including all necessary structures, functions and modules used to build a new plugin.
    ///
    /// It should be included using:
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    /// ```
    pub mod build_plugin {
        pub use crate::{
            generator::{generate_at_rules, generate_class, generate_wrapper, ContextCanHandle, ContextHandle},
            plugins::{
                Plugin, StaticPlugin, DynamicPlugin, PluginArbitraryMatcher, PluginArbitraryMatcherModifier,
                PluginKind, PropertyName, PropertyName::*, StaticPropertyName, DynamicPropertyName,
                StaticPluginKind, DynamicPluginKind,
            },
            selector::{Modifier, ArbitraryHint},
            utils::{buffer::Buffer, color, shadow, spacing, value_matchers::*},
        };
        pub use phf::{self, phf_map};
        pub use std::fmt::{self, Write};
    }
}
