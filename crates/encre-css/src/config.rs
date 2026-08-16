//! Define the [`Config`] structure used to configure the [`generate`] function using a
//! [Tailwind-like configuration](https://tailwindcss.com/docs/configuration).
//!
//! # Example
//!
//! ```
//! use encre_css::{Config, config::DarkMode};
//!
//! let mut config = Config::default();
//!
//! // Toggles the dark mode using the class `.dark`
//! config.theme.dark_mode = DarkMode::new_class(".dark");
//!
//! // Defines some custom colors, they will be usable in the `text`, `bg`,
//! // `border`, etc utilities.
//! config.theme.colors.add("primary", "#d3198c");
//! config.theme.colors.add("secondary", "#fff");
//!
//! // Defines some custom screen breakpoints
//! config.theme.screens.add("tablet", "640px");
//! config.theme.screens.add("laptop", "1024px");
//! config.theme.screens.add("desktop", "1280px");
//!
//! let generated = encre_css::generate(
//!     ["tablet:dark:bg-primary"],
//!     &config,
//! );
//!
//! assert!(generated.ends_with(r#"@media (width >= 640px) {
//!   .dark .tablet\:dark\:bg-primary {
//!     background-color: #d3198c;
//!   }
//! }"#));
//! ```
//!
//! The previous example is equivalent to the following TOML configuration file:
//!
//! <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[theme]</span>
//! dark_mode = { class = <span class="string">".dark"</span> }
//! colors = { primary = <span class="string">"#d3198c"</span>, secondary = <span class="string">"#fff"</span> }
//! screens = { tablet = <span class="string">"640px"</span>, laptop = <span class="string">"1024px"</span>, desktop = <span class="string">"1280px"</span> }
//! </code></pre></div>
//!
//! [`generate`]: crate::generate
use crate::{
    error::{Error, Result},
    preflight::Preflight,
    scanner::Scanner,
    selector::Variant,
};

#[allow(clippy::wildcard_imports)]
use crate::plugins::*;

use phf::{phf_map as map, phf_ordered_map};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt, fs, iter,
    path::Path,
};

/// The list of all default colors.
///
/// <table style="display: table;">
///     <thead>
///         <tr>
///             <th style="text-align: center;">Name</th>
///             <th style="text-align: center;">RGB Value</th>
///             <th style="text-align: center;">Color</th>
///         </tr>
///     </thead>
///     <tbody>
///         <tr><td>red-50</td><td>oklch(97.1% .013 17.38)</td><td style="background-color: oklch(97.1% .013 17.38);"></td></tr>
///         <tr><td>red-100</td><td>oklch(93.6% .032 17.717)</td><td style="background-color: oklch(93.6% .032 17.717);"></td></tr>
///         <tr><td>red-200</td><td>oklch(88.5% .062 18.334)</td><td style="background-color: oklch(88.5% .062 18.334);"></td></tr>
///         <tr><td>red-300</td><td>oklch(80.8% .114 19.571)</td><td style="background-color: oklch(80.8% .114 19.571);"></td></tr>
///         <tr><td>red-400</td><td>oklch(70.4% .191 22.216)</td><td style="background-color: oklch(70.4% .191 22.216);"></td></tr>
///         <tr><td>red-500</td><td>oklch(63.7% .237 25.331)</td><td style="background-color: oklch(63.7% .237 25.331);"></td></tr>
///         <tr><td>red-600</td><td>oklch(57.7% .245 27.325)</td><td style="background-color: oklch(57.7% .245 27.325);"></td></tr>
///         <tr><td>red-700</td><td>oklch(50.5% .213 27.518)</td><td style="background-color: oklch(50.5% .213 27.518);"></td></tr>
///         <tr><td>red-800</td><td>oklch(44.4% .177 26.899)</td><td style="background-color: oklch(44.4% .177 26.899);"></td></tr>
///         <tr><td>red-900</td><td>oklch(39.6% .141 25.723)</td><td style="background-color: oklch(39.6% .141 25.723);"></td></tr>
///         <tr><td>red-950</td><td>oklch(25.8% .092 26.042)</td><td style="background-color: oklch(25.8% .092 26.042);"></td></tr>
///         <tr><td>orange-50</td><td>oklch(98% .016 73.684)</td><td style="background-color: oklch(98% .016 73.684);"></td></tr>
///         <tr><td>orange-100</td><td>oklch(95.4% .038 75.164)</td><td style="background-color: oklch(95.4% .038 75.164);"></td></tr>
///         <tr><td>orange-200</td><td>oklch(90.1% .076 70.697)</td><td style="background-color: oklch(90.1% .076 70.697);"></td></tr>
///         <tr><td>orange-300</td><td>oklch(83.7% .128 66.29)</td><td style="background-color: oklch(83.7% .128 66.29);"></td></tr>
///         <tr><td>orange-400</td><td>oklch(75% .183 55.934)</td><td style="background-color: oklch(75% .183 55.934);"></td></tr>
///         <tr><td>orange-500</td><td>oklch(70.5% .213 47.604)</td><td style="background-color: oklch(70.5% .213 47.604);"></td></tr>
///         <tr><td>orange-600</td><td>oklch(64.6% .222 41.116)</td><td style="background-color: oklch(64.6% .222 41.116);"></td></tr>
///         <tr><td>orange-700</td><td>oklch(55.3% .195 38.402)</td><td style="background-color: oklch(55.3% .195 38.402);"></td></tr>
///         <tr><td>orange-800</td><td>oklch(47% .157 37.304)</td><td style="background-color: oklch(47% .157 37.304);"></td></tr>
///         <tr><td>orange-900</td><td>oklch(40.8% .123 38.172)</td><td style="background-color: oklch(40.8% .123 38.172);"></td></tr>
///         <tr><td>orange-950</td><td>oklch(26.6% .079 36.259)</td><td style="background-color: oklch(26.6% .079 36.259);"></td></tr>
///         <tr><td>amber-50</td><td>oklch(98.7% .022 95.277)</td><td style="background-color: oklch(98.7% .022 95.277);"></td></tr>
///         <tr><td>amber-100</td><td>oklch(96.2% .059 95.617)</td><td style="background-color: oklch(96.2% .059 95.617);"></td></tr>
///         <tr><td>amber-200</td><td>oklch(92.4% .12 95.746)</td><td style="background-color: oklch(92.4% .12 95.746);"></td></tr>
///         <tr><td>amber-300</td><td>oklch(87.9% .169 91.605)</td><td style="background-color: oklch(87.9% .169 91.605);"></td></tr>
///         <tr><td>amber-400</td><td>oklch(82.8% .189 84.429)</td><td style="background-color: oklch(82.8% .189 84.429);"></td></tr>
///         <tr><td>amber-500</td><td>oklch(76.9% .188 70.08)</td><td style="background-color: oklch(76.9% .188 70.08);"></td></tr>
///         <tr><td>amber-600</td><td>oklch(66.6% .179 58.318)</td><td style="background-color: oklch(66.6% .179 58.318);"></td></tr>
///         <tr><td>amber-700</td><td>oklch(55.5% .163 48.998)</td><td style="background-color: oklch(55.5% .163 48.998);"></td></tr>
///         <tr><td>amber-800</td><td>oklch(47.3% .137 46.201)</td><td style="background-color: oklch(47.3% .137 46.201);"></td></tr>
///         <tr><td>amber-900</td><td>oklch(41.4% .112 45.904)</td><td style="background-color: oklch(41.4% .112 45.904);"></td></tr>
///         <tr><td>amber-950</td><td>oklch(27.9% .077 45.635)</td><td style="background-color: oklch(27.9% .077 45.635);"></td></tr>
///         <tr><td>yellow-50</td><td>oklch(98.7% .026 102.212)</td><td style="background-color: oklch(98.7% .026 102.212);"></td></tr>
///         <tr><td>yellow-100</td><td>oklch(97.3% .071 103.193)</td><td style="background-color: oklch(97.3% .071 103.193);"></td></tr>
///         <tr><td>yellow-200</td><td>oklch(94.5% .129 101.54)</td><td style="background-color: oklch(94.5% .129 101.54);"></td></tr>
///         <tr><td>yellow-300</td><td>oklch(90.5% .182 98.111)</td><td style="background-color: oklch(90.5% .182 98.111);"></td></tr>
///         <tr><td>yellow-400</td><td>oklch(85.2% .199 91.936)</td><td style="background-color: oklch(85.2% .199 91.936);"></td></tr>
///         <tr><td>yellow-500</td><td>oklch(79.5% .184 86.047)</td><td style="background-color: oklch(79.5% .184 86.047);"></td></tr>
///         <tr><td>yellow-600</td><td>oklch(68.1% .162 75.834)</td><td style="background-color: oklch(68.1% .162 75.834);"></td></tr>
///         <tr><td>yellow-700</td><td>oklch(55.4% .135 66.442)</td><td style="background-color: oklch(55.4% .135 66.442);"></td></tr>
///         <tr><td>yellow-800</td><td>oklch(47.6% .114 61.907)</td><td style="background-color: oklch(47.6% .114 61.907);"></td></tr>
///         <tr><td>yellow-900</td><td>oklch(42.1% .095 57.708)</td><td style="background-color: oklch(42.1% .095 57.708);"></td></tr>
///         <tr><td>yellow-950</td><td>oklch(28.6% .066 53.813)</td><td style="background-color: oklch(28.6% .066 53.813);"></td></tr>
///         <tr><td>lime-50</td><td>oklch(98.6% .031 120.757)</td><td style="background-color: oklch(98.6% .031 120.757);"></td></tr>
///         <tr><td>lime-100</td><td>oklch(96.7% .067 122.328)</td><td style="background-color: oklch(96.7% .067 122.328);"></td></tr>
///         <tr><td>lime-200</td><td>oklch(93.8% .127 124.321)</td><td style="background-color: oklch(93.8% .127 124.321);"></td></tr>
///         <tr><td>lime-300</td><td>oklch(89.7% .196 126.665)</td><td style="background-color: oklch(89.7% .196 126.665);"></td></tr>
///         <tr><td>lime-400</td><td>oklch(84.1% .238 128.85)</td><td style="background-color: oklch(84.1% .238 128.85);"></td></tr>
///         <tr><td>lime-500</td><td>oklch(76.8% .233 130.85)</td><td style="background-color: oklch(76.8% .233 130.85);"></td></tr>
///         <tr><td>lime-600</td><td>oklch(64.8% .2 131.684)</td><td style="background-color: oklch(64.8% .2 131.684);"></td></tr>
///         <tr><td>lime-700</td><td>oklch(53.2% .157 131.589)</td><td style="background-color: oklch(53.2% .157 131.589);"></td></tr>
///         <tr><td>lime-800</td><td>oklch(45.3% .124 130.933)</td><td style="background-color: oklch(45.3% .124 130.933);"></td></tr>
///         <tr><td>lime-900</td><td>oklch(40.5% .101 131.063)</td><td style="background-color: oklch(40.5% .101 131.063);"></td></tr>
///         <tr><td>lime-950</td><td>oklch(27.4% .072 132.109)</td><td style="background-color: oklch(27.4% .072 132.109);"></td></tr>
///         <tr><td>green-50</td><td>oklch(98.2% .018 155.826)</td><td style="background-color: oklch(98.2% .018 155.826);"></td></tr>
///         <tr><td>green-100</td><td>oklch(96.2% .044 156.743)</td><td style="background-color: oklch(96.2% .044 156.743);"></td></tr>
///         <tr><td>green-200</td><td>oklch(92.5% .084 155.995)</td><td style="background-color: oklch(92.5% .084 155.995);"></td></tr>
///         <tr><td>green-300</td><td>oklch(87.1% .15 154.449)</td><td style="background-color: oklch(87.1% .15 154.449);"></td></tr>
///         <tr><td>green-400</td><td>oklch(79.2% .209 151.711)</td><td style="background-color: oklch(79.2% .209 151.711);"></td></tr>
///         <tr><td>green-500</td><td>oklch(72.3% .219 149.579)</td><td style="background-color: oklch(72.3% .219 149.579);"></td></tr>
///         <tr><td>green-600</td><td>oklch(62.7% .194 149.214)</td><td style="background-color: oklch(62.7% .194 149.214);"></td></tr>
///         <tr><td>green-700</td><td>oklch(52.7% .154 150.069)</td><td style="background-color: oklch(52.7% .154 150.069);"></td></tr>
///         <tr><td>green-800</td><td>oklch(44.8% .119 151.328)</td><td style="background-color: oklch(44.8% .119 151.328);"></td></tr>
///         <tr><td>green-900</td><td>oklch(39.3% .095 152.535)</td><td style="background-color: oklch(39.3% .095 152.535);"></td></tr>
///         <tr><td>green-950</td><td>oklch(26.6% .065 152.934)</td><td style="background-color: oklch(26.6% .065 152.934);"></td></tr>
///         <tr><td>emerald-50</td><td>oklch(97.9% .021 166.113)</td><td style="background-color: oklch(97.9% .021 166.113);"></td></tr>
///         <tr><td>emerald-100</td><td>oklch(95% .052 163.051)</td><td style="background-color: oklch(95% .052 163.051);"></td></tr>
///         <tr><td>emerald-200</td><td>oklch(90.5% .093 164.15)</td><td style="background-color: oklch(90.5% .093 164.15);"></td></tr>
///         <tr><td>emerald-300</td><td>oklch(84.5% .143 164.978)</td><td style="background-color: oklch(84.5% .143 164.978);"></td></tr>
///         <tr><td>emerald-400</td><td>oklch(76.5% .177 163.223)</td><td style="background-color: oklch(76.5% .177 163.223);"></td></tr>
///         <tr><td>emerald-500</td><td>oklch(69.6% .17 162.48)</td><td style="background-color: oklch(69.6% .17 162.48);"></td></tr>
///         <tr><td>emerald-600</td><td>oklch(59.6% .145 163.225)</td><td style="background-color: oklch(59.6% .145 163.225);"></td></tr>
///         <tr><td>emerald-700</td><td>oklch(50.8% .118 165.612)</td><td style="background-color: oklch(50.8% .118 165.612);"></td></tr>
///         <tr><td>emerald-800</td><td>oklch(43.2% .095 166.913)</td><td style="background-color: oklch(43.2% .095 166.913);"></td></tr>
///         <tr><td>emerald-900</td><td>oklch(37.8% .077 168.94)</td><td style="background-color: oklch(37.8% .077 168.94);"></td></tr>
///         <tr><td>emerald-950</td><td>oklch(26.2% .051 172.552)</td><td style="background-color: oklch(26.2% .051 172.552);"></td></tr>
///         <tr><td>teal-50</td><td>oklch(98.4% .014 180.72)</td><td style="background-color: oklch(98.4% .014 180.72);"></td></tr>
///         <tr><td>teal-100</td><td>oklch(95.3% .051 180.801)</td><td style="background-color: oklch(95.3% .051 180.801);"></td></tr>
///         <tr><td>teal-200</td><td>oklch(91% .096 180.426)</td><td style="background-color: oklch(91% .096 180.426);"></td></tr>
///         <tr><td>teal-300</td><td>oklch(85.5% .138 181.071)</td><td style="background-color: oklch(85.5% .138 181.071);"></td></tr>
///         <tr><td>teal-400</td><td>oklch(77.7% .152 181.912)</td><td style="background-color: oklch(77.7% .152 181.912);"></td></tr>
///         <tr><td>teal-500</td><td>oklch(70.4% .14 182.503)</td><td style="background-color: oklch(70.4% .14 182.503);"></td></tr>
///         <tr><td>teal-600</td><td>oklch(60% .118 184.704)</td><td style="background-color: oklch(60% .118 184.704);"></td></tr>
///         <tr><td>teal-700</td><td>oklch(51.1% .096 186.391)</td><td style="background-color: oklch(51.1% .096 186.391);"></td></tr>
///         <tr><td>teal-800</td><td>oklch(43.7% .078 188.216)</td><td style="background-color: oklch(43.7% .078 188.216);"></td></tr>
///         <tr><td>teal-900</td><td>oklch(38.6% .063 188.416)</td><td style="background-color: oklch(38.6% .063 188.416);"></td></tr>
///         <tr><td>teal-950</td><td>oklch(27.7% .046 192.524)</td><td style="background-color: oklch(27.7% .046 192.524);"></td></tr>
///         <tr><td>cyan-50</td><td>oklch(98.4% .019 200.873)</td><td style="background-color: oklch(98.4% .019 200.873);"></td></tr>
///         <tr><td>cyan-100</td><td>oklch(95.6% .045 203.388)</td><td style="background-color: oklch(95.6% .045 203.388);"></td></tr>
///         <tr><td>cyan-200</td><td>oklch(91.7% .08 205.041)</td><td style="background-color: oklch(91.7% .08 205.041);"></td></tr>
///         <tr><td>cyan-300</td><td>oklch(86.5% .127 207.078)</td><td style="background-color: oklch(86.5% .127 207.078);"></td></tr>
///         <tr><td>cyan-400</td><td>oklch(78.9% .154 211.53)</td><td style="background-color: oklch(78.9% .154 211.53);"></td></tr>
///         <tr><td>cyan-500</td><td>oklch(71.5% .143 215.221)</td><td style="background-color: oklch(71.5% .143 215.221);"></td></tr>
///         <tr><td>cyan-600</td><td>oklch(60.9% .126 221.723)</td><td style="background-color: oklch(60.9% .126 221.723);"></td></tr>
///         <tr><td>cyan-700</td><td>oklch(52% .105 223.128)</td><td style="background-color: oklch(52% .105 223.128);"></td></tr>
///         <tr><td>cyan-800</td><td>oklch(45% .085 224.283)</td><td style="background-color: oklch(45% .085 224.283);"></td></tr>
///         <tr><td>cyan-900</td><td>oklch(39.8% .07 227.392)</td><td style="background-color: oklch(39.8% .07 227.392);"></td></tr>
///         <tr><td>cyan-950</td><td>oklch(30.2% .056 229.695)</td><td style="background-color: oklch(30.2% .056 229.695);"></td></tr>
///         <tr><td>sky-50</td><td>oklch(97.7% .013 236.62)</td><td style="background-color: oklch(97.7% .013 236.62);"></td></tr>
///         <tr><td>sky-100</td><td>oklch(95.1% .026 236.824)</td><td style="background-color: oklch(95.1% .026 236.824);"></td></tr>
///         <tr><td>sky-200</td><td>oklch(90.1% .058 230.902)</td><td style="background-color: oklch(90.1% .058 230.902);"></td></tr>
///         <tr><td>sky-300</td><td>oklch(82.8% .111 230.318)</td><td style="background-color: oklch(82.8% .111 230.318);"></td></tr>
///         <tr><td>sky-400</td><td>oklch(74.6% .16 232.661)</td><td style="background-color: oklch(74.6% .16 232.661);"></td></tr>
///         <tr><td>sky-500</td><td>oklch(68.5% .169 237.323)</td><td style="background-color: oklch(68.5% .169 237.323);"></td></tr>
///         <tr><td>sky-600</td><td>oklch(58.8% .158 241.966)</td><td style="background-color: oklch(58.8% .158 241.966);"></td></tr>
///         <tr><td>sky-700</td><td>oklch(50% .134 242.749)</td><td style="background-color: oklch(50% .134 242.749);"></td></tr>
///         <tr><td>sky-800</td><td>oklch(44.3% .11 240.79)</td><td style="background-color: oklch(44.3% .11 240.79);"></td></tr>
///         <tr><td>sky-900</td><td>oklch(39.1% .09 240.876)</td><td style="background-color: oklch(39.1% .09 240.876);"></td></tr>
///         <tr><td>sky-950</td><td>oklch(29.3% .066 243.157)</td><td style="background-color: oklch(29.3% .066 243.157);"></td></tr>
///         <tr><td>blue-50</td><td>oklch(97% .014 254.604)</td><td style="background-color: oklch(97% .014 254.604);"></td></tr>
///         <tr><td>blue-100</td><td>oklch(93.2% .032 255.585)</td><td style="background-color: oklch(93.2% .032 255.585);"></td></tr>
///         <tr><td>blue-200</td><td>oklch(88.2% .059 254.128)</td><td style="background-color: oklch(88.2% .059 254.128);"></td></tr>
///         <tr><td>blue-300</td><td>oklch(80.9% .105 251.813)</td><td style="background-color: oklch(80.9% .105 251.813);"></td></tr>
///         <tr><td>blue-400</td><td>oklch(70.7% .165 254.624)</td><td style="background-color: oklch(70.7% .165 254.624);"></td></tr>
///         <tr><td>blue-500</td><td>oklch(62.3% .214 259.815)</td><td style="background-color: oklch(62.3% .214 259.815);"></td></tr>
///         <tr><td>blue-600</td><td>oklch(54.6% .245 262.881)</td><td style="background-color: oklch(54.6% .245 262.881);"></td></tr>
///         <tr><td>blue-700</td><td>oklch(48.8% .243 264.376)</td><td style="background-color: oklch(48.8% .243 264.376);"></td></tr>
///         <tr><td>blue-800</td><td>oklch(42.4% .199 265.638)</td><td style="background-color: oklch(42.4% .199 265.638);"></td></tr>
///         <tr><td>blue-900</td><td>oklch(37.9% .146 265.522)</td><td style="background-color: oklch(37.9% .146 265.522);"></td></tr>
///         <tr><td>blue-950</td><td>oklch(28.2% .091 267.935)</td><td style="background-color: oklch(28.2% .091 267.935);"></td></tr>
///         <tr><td>indigo-50</td><td>oklch(96.2% .018 272.314)</td><td style="background-color: oklch(96.2% .018 272.314);"></td></tr>
///         <tr><td>indigo-100</td><td>oklch(93% .034 272.788)</td><td style="background-color: oklch(93% .034 272.788);"></td></tr>
///         <tr><td>indigo-200</td><td>oklch(87% .065 274.039)</td><td style="background-color: oklch(87% .065 274.039);"></td></tr>
///         <tr><td>indigo-300</td><td>oklch(78.5% .115 274.713)</td><td style="background-color: oklch(78.5% .115 274.713);"></td></tr>
///         <tr><td>indigo-400</td><td>oklch(67.3% .182 276.935)</td><td style="background-color: oklch(67.3% .182 276.935);"></td></tr>
///         <tr><td>indigo-500</td><td>oklch(58.5% .233 277.117)</td><td style="background-color: oklch(58.5% .233 277.117);"></td></tr>
///         <tr><td>indigo-600</td><td>oklch(51.1% .262 276.966)</td><td style="background-color: oklch(51.1% .262 276.966);"></td></tr>
///         <tr><td>indigo-700</td><td>oklch(45.7% .24 277.023)</td><td style="background-color: oklch(45.7% .24 277.023);"></td></tr>
///         <tr><td>indigo-800</td><td>oklch(39.8% .195 277.366)</td><td style="background-color: oklch(39.8% .195 277.366);"></td></tr>
///         <tr><td>indigo-900</td><td>oklch(35.9% .144 278.697)</td><td style="background-color: oklch(35.9% .144 278.697);"></td></tr>
///         <tr><td>indigo-950</td><td>oklch(25.7% .09 281.288)</td><td style="background-color: oklch(25.7% .09 281.288);"></td></tr>
///         <tr><td>violet-50</td><td>oklch(96.9% .016 293.756)</td><td style="background-color: oklch(96.9% .016 293.756);"></td></tr>
///         <tr><td>violet-100</td><td>oklch(94.3% .029 294.588)</td><td style="background-color: oklch(94.3% .029 294.588);"></td></tr>
///         <tr><td>violet-200</td><td>oklch(89.4% .057 293.283)</td><td style="background-color: oklch(89.4% .057 293.283);"></td></tr>
///         <tr><td>violet-300</td><td>oklch(81.1% .111 293.571)</td><td style="background-color: oklch(81.1% .111 293.571);"></td></tr>
///         <tr><td>violet-400</td><td>oklch(70.2% .183 293.541)</td><td style="background-color: oklch(70.2% .183 293.541);"></td></tr>
///         <tr><td>violet-500</td><td>oklch(60.6% .25 292.717)</td><td style="background-color: oklch(60.6% .25 292.717);"></td></tr>
///         <tr><td>violet-600</td><td>oklch(54.1% .281 293.009)</td><td style="background-color: oklch(54.1% .281 293.009);"></td></tr>
///         <tr><td>violet-700</td><td>oklch(49.1% .27 292.581)</td><td style="background-color: oklch(49.1% .27 292.581);"></td></tr>
///         <tr><td>violet-800</td><td>oklch(43.2% .232 292.759)</td><td style="background-color: oklch(43.2% .232 292.759);"></td></tr>
///         <tr><td>violet-900</td><td>oklch(38% .189 293.745)</td><td style="background-color: oklch(38% .189 293.745);"></td></tr>
///         <tr><td>violet-950</td><td>oklch(28.3% .141 291.089)</td><td style="background-color: oklch(28.3% .141 291.089);"></td></tr>
///         <tr><td>purple-50</td><td>oklch(97.7% .014 308.299)</td><td style="background-color: oklch(97.7% .014 308.299);"></td></tr>
///         <tr><td>purple-100</td><td>oklch(94.6% .033 307.174)</td><td style="background-color: oklch(94.6% .033 307.174);"></td></tr>
///         <tr><td>purple-200</td><td>oklch(90.2% .063 306.703)</td><td style="background-color: oklch(90.2% .063 306.703);"></td></tr>
///         <tr><td>purple-300</td><td>oklch(82.7% .119 306.383)</td><td style="background-color: oklch(82.7% .119 306.383);"></td></tr>
///         <tr><td>purple-400</td><td>oklch(71.4% .203 305.504)</td><td style="background-color: oklch(71.4% .203 305.504);"></td></tr>
///         <tr><td>purple-500</td><td>oklch(62.7% .265 303.9)</td><td style="background-color: oklch(62.7% .265 303.9);"></td></tr>
///         <tr><td>purple-600</td><td>oklch(55.8% .288 302.321)</td><td style="background-color: oklch(55.8% .288 302.321);"></td></tr>
///         <tr><td>purple-700</td><td>oklch(49.6% .265 301.924)</td><td style="background-color: oklch(49.6% .265 301.924);"></td></tr>
///         <tr><td>purple-800</td><td>oklch(43.8% .218 303.724)</td><td style="background-color: oklch(43.8% .218 303.724);"></td></tr>
///         <tr><td>purple-900</td><td>oklch(38.1% .176 304.987)</td><td style="background-color: oklch(38.1% .176 304.987);"></td></tr>
///         <tr><td>purple-950</td><td>oklch(29.1% .149 302.717)</td><td style="background-color: oklch(29.1% .149 302.717);"></td></tr>
///         <tr><td>fuchsia-50</td><td>oklch(97.7% .017 320.058)</td><td style="background-color: oklch(97.7% .017 320.058);"></td></tr>
///         <tr><td>fuchsia-100</td><td>oklch(95.2% .037 318.852)</td><td style="background-color: oklch(95.2% .037 318.852);"></td></tr>
///         <tr><td>fuchsia-200</td><td>oklch(90.3% .076 319.62)</td><td style="background-color: oklch(90.3% .076 319.62);"></td></tr>
///         <tr><td>fuchsia-300</td><td>oklch(83.3% .145 321.434)</td><td style="background-color: oklch(83.3% .145 321.434);"></td></tr>
///         <tr><td>fuchsia-400</td><td>oklch(74% .238 322.16)</td><td style="background-color: oklch(74% .238 322.16);"></td></tr>
///         <tr><td>fuchsia-500</td><td>oklch(66.7% .295 322.15)</td><td style="background-color: oklch(66.7% .295 322.15);"></td></tr>
///         <tr><td>fuchsia-600</td><td>oklch(59.1% .293 322.896)</td><td style="background-color: oklch(59.1% .293 322.896);"></td></tr>
///         <tr><td>fuchsia-700</td><td>oklch(51.8% .253 323.949)</td><td style="background-color: oklch(51.8% .253 323.949);"></td></tr>
///         <tr><td>fuchsia-800</td><td>oklch(45.2% .211 324.591)</td><td style="background-color: oklch(45.2% .211 324.591);"></td></tr>
///         <tr><td>fuchsia-900</td><td>oklch(40.1% .17 325.612)</td><td style="background-color: oklch(40.1% .17 325.612);"></td></tr>
///         <tr><td>fuchsia-950</td><td>oklch(29.3% .136 325.661)</td><td style="background-color: oklch(29.3% .136 325.661);"></td></tr>
///         <tr><td>pink-50</td><td>oklch(97.1% .014 343.198)</td><td style="background-color: oklch(97.1% .014 343.198);"></td></tr>
///         <tr><td>pink-100</td><td>oklch(94.8% .028 342.258)</td><td style="background-color: oklch(94.8% .028 342.258);"></td></tr>
///         <tr><td>pink-200</td><td>oklch(89.9% .061 343.231)</td><td style="background-color: oklch(89.9% .061 343.231);"></td></tr>
///         <tr><td>pink-300</td><td>oklch(82.3% .12 346.018)</td><td style="background-color: oklch(82.3% .12 346.018);"></td></tr>
///         <tr><td>pink-400</td><td>oklch(71.8% .202 349.761)</td><td style="background-color: oklch(71.8% .202 349.761);"></td></tr>
///         <tr><td>pink-500</td><td>oklch(65.6% .241 354.308)</td><td style="background-color: oklch(65.6% .241 354.308);"></td></tr>
///         <tr><td>pink-600</td><td>oklch(59.2% .249 .584)</td><td style="background-color: oklch(59.2% .249 .584);"></td></tr>
///         <tr><td>pink-700</td><td>oklch(52.5% .223 3.958)</td><td style="background-color: oklch(52.5% .223 3.958);"></td></tr>
///         <tr><td>pink-800</td><td>oklch(45.9% .187 3.815)</td><td style="background-color: oklch(45.9% .187 3.815);"></td></tr>
///         <tr><td>pink-900</td><td>oklch(40.8% .153 2.432)</td><td style="background-color: oklch(40.8% .153 2.432);"></td></tr>
///         <tr><td>pink-950</td><td>oklch(28.4% .109 3.907)</td><td style="background-color: oklch(28.4% .109 3.907);"></td></tr>
///         <tr><td>rose-50</td><td>oklch(96.9% .015 12.422)</td><td style="background-color: oklch(96.9% .015 12.422);"></td></tr>
///         <tr><td>rose-100</td><td>oklch(94.1% .03 12.58)</td><td style="background-color: oklch(94.1% .03 12.58);"></td></tr>
///         <tr><td>rose-200</td><td>oklch(89.2% .058 10.001)</td><td style="background-color: oklch(89.2% .058 10.001);"></td></tr>
///         <tr><td>rose-300</td><td>oklch(81% .117 11.638)</td><td style="background-color: oklch(81% .117 11.638);"></td></tr>
///         <tr><td>rose-400</td><td>oklch(71.2% .194 13.428)</td><td style="background-color: oklch(71.2% .194 13.428);"></td></tr>
///         <tr><td>rose-500</td><td>oklch(64.5% .246 16.439)</td><td style="background-color: oklch(64.5% .246 16.439);"></td></tr>
///         <tr><td>rose-600</td><td>oklch(58.6% .253 17.585)</td><td style="background-color: oklch(58.6% .253 17.585);"></td></tr>
///         <tr><td>rose-700</td><td>oklch(51.4% .222 16.935)</td><td style="background-color: oklch(51.4% .222 16.935);"></td></tr>
///         <tr><td>rose-800</td><td>oklch(45.5% .188 13.697)</td><td style="background-color: oklch(45.5% .188 13.697);"></td></tr>
///         <tr><td>rose-900</td><td>oklch(41% .159 10.272)</td><td style="background-color: oklch(41% .159 10.272);"></td></tr>
///         <tr><td>rose-950</td><td>oklch(27.1% .105 12.094)</td><td style="background-color: oklch(27.1% .105 12.094);"></td></tr>
///         <tr><td>slate-50</td><td>oklch(98.4% .003 247.858)</td><td style="background-color: oklch(98.4% .003 247.858);"></td></tr>
///         <tr><td>slate-100</td><td>oklch(96.8% .007 247.896)</td><td style="background-color: oklch(96.8% .007 247.896);"></td></tr>
///         <tr><td>slate-200</td><td>oklch(92.9% .013 255.508)</td><td style="background-color: oklch(92.9% .013 255.508);"></td></tr>
///         <tr><td>slate-300</td><td>oklch(86.9% .022 252.894)</td><td style="background-color: oklch(86.9% .022 252.894);"></td></tr>
///         <tr><td>slate-400</td><td>oklch(70.4% .04 256.788)</td><td style="background-color: oklch(70.4% .04 256.788);"></td></tr>
///         <tr><td>slate-500</td><td>oklch(55.4% .046 257.417)</td><td style="background-color: oklch(55.4% .046 257.417);"></td></tr>
///         <tr><td>slate-600</td><td>oklch(44.6% .043 257.281)</td><td style="background-color: oklch(44.6% .043 257.281);"></td></tr>
///         <tr><td>slate-700</td><td>oklch(37.2% .044 257.287)</td><td style="background-color: oklch(37.2% .044 257.287);"></td></tr>
///         <tr><td>slate-800</td><td>oklch(27.9% .041 260.031)</td><td style="background-color: oklch(27.9% .041 260.031);"></td></tr>
///         <tr><td>slate-900</td><td>oklch(20.8% .042 265.755)</td><td style="background-color: oklch(20.8% .042 265.755);"></td></tr>
///         <tr><td>slate-950</td><td>oklch(12.9% .042 264.695)</td><td style="background-color: oklch(12.9% .042 264.695);"></td></tr>
///         <tr><td>gray-50</td><td>oklch(98.5% .002 247.839)</td><td style="background-color: oklch(98.5% .002 247.839);"></td></tr>
///         <tr><td>gray-100</td><td>oklch(96.7% .003 264.542)</td><td style="background-color: oklch(96.7% .003 264.542);"></td></tr>
///         <tr><td>gray-200</td><td>oklch(92.8% .006 264.531)</td><td style="background-color: oklch(92.8% .006 264.531);"></td></tr>
///         <tr><td>gray-300</td><td>oklch(87.2% .01 258.338)</td><td style="background-color: oklch(87.2% .01 258.338);"></td></tr>
///         <tr><td>gray-400</td><td>oklch(70.7% .022 261.325)</td><td style="background-color: oklch(70.7% .022 261.325);"></td></tr>
///         <tr><td>gray-500</td><td>oklch(55.1% .027 264.364)</td><td style="background-color: oklch(55.1% .027 264.364);"></td></tr>
///         <tr><td>gray-600</td><td>oklch(44.6% .03 256.802)</td><td style="background-color: oklch(44.6% .03 256.802);"></td></tr>
///         <tr><td>gray-700</td><td>oklch(37.3% .034 259.733)</td><td style="background-color: oklch(37.3% .034 259.733);"></td></tr>
///         <tr><td>gray-800</td><td>oklch(27.8% .033 256.848)</td><td style="background-color: oklch(27.8% .033 256.848);"></td></tr>
///         <tr><td>gray-900</td><td>oklch(21% .034 264.665)</td><td style="background-color: oklch(21% .034 264.665);"></td></tr>
///         <tr><td>gray-950</td><td>oklch(13% .028 261.692)</td><td style="background-color: oklch(13% .028 261.692);"></td></tr>
///         <tr><td>zinc-50</td><td>oklch(98.5% 0 0)</td><td style="background-color: oklch(98.5% 0 0);"></td></tr>
///         <tr><td>zinc-100</td><td>oklch(96.7% .001 286.375)</td><td style="background-color: oklch(96.7% .001 286.375);"></td></tr>
///         <tr><td>zinc-200</td><td>oklch(92% .004 286.32)</td><td style="background-color: oklch(92% .004 286.32);"></td></tr>
///         <tr><td>zinc-300</td><td>oklch(87.1% .006 286.286)</td><td style="background-color: oklch(87.1% .006 286.286);"></td></tr>
///         <tr><td>zinc-400</td><td>oklch(70.5% .015 286.067)</td><td style="background-color: oklch(70.5% .015 286.067);"></td></tr>
///         <tr><td>zinc-500</td><td>oklch(55.2% .016 285.938)</td><td style="background-color: oklch(55.2% .016 285.938);"></td></tr>
///         <tr><td>zinc-600</td><td>oklch(44.2% .017 285.786)</td><td style="background-color: oklch(44.2% .017 285.786);"></td></tr>
///         <tr><td>zinc-700</td><td>oklch(37% .013 285.805)</td><td style="background-color: oklch(37% .013 285.805);"></td></tr>
///         <tr><td>zinc-800</td><td>oklch(27.4% .006 286.033)</td><td style="background-color: oklch(27.4% .006 286.033);"></td></tr>
///         <tr><td>zinc-900</td><td>oklch(21% .006 285.885)</td><td style="background-color: oklch(21% .006 285.885);"></td></tr>
///         <tr><td>zinc-950</td><td>oklch(14.1% .005 285.823)</td><td style="background-color: oklch(14.1% .005 285.823);"></td></tr>
///         <tr><td>neutral-50</td><td>oklch(98.5% 0 0)</td><td style="background-color: oklch(98.5% 0 0);"></td></tr>
///         <tr><td>neutral-100</td><td>oklch(97% 0 0)</td><td style="background-color: oklch(97% 0 0);"></td></tr>
///         <tr><td>neutral-200</td><td>oklch(92.2% 0 0)</td><td style="background-color: oklch(92.2% 0 0);"></td></tr>
///         <tr><td>neutral-300</td><td>oklch(87% 0 0)</td><td style="background-color: oklch(87% 0 0);"></td></tr>
///         <tr><td>neutral-400</td><td>oklch(70.8% 0 0)</td><td style="background-color: oklch(70.8% 0 0);"></td></tr>
///         <tr><td>neutral-500</td><td>oklch(55.6% 0 0)</td><td style="background-color: oklch(55.6% 0 0);"></td></tr>
///         <tr><td>neutral-600</td><td>oklch(43.9% 0 0)</td><td style="background-color: oklch(43.9% 0 0);"></td></tr>
///         <tr><td>neutral-700</td><td>oklch(37.1% 0 0)</td><td style="background-color: oklch(37.1% 0 0);"></td></tr>
///         <tr><td>neutral-800</td><td>oklch(26.9% 0 0)</td><td style="background-color: oklch(26.9% 0 0);"></td></tr>
///         <tr><td>neutral-900</td><td>oklch(20.5% 0 0)</td><td style="background-color: oklch(20.5% 0 0);"></td></tr>
///         <tr><td>neutral-950</td><td>oklch(14.5% 0 0)</td><td style="background-color: oklch(14.5% 0 0);"></td></tr>
///         <tr><td>stone-50</td><td>oklch(98.5% .001 106.423)</td><td style="background-color: oklch(98.5% .001 106.423);"></td></tr>
///         <tr><td>stone-100</td><td>oklch(97% .001 106.424)</td><td style="background-color: oklch(97% .001 106.424);"></td></tr>
///         <tr><td>stone-200</td><td>oklch(92.3% .003 48.717)</td><td style="background-color: oklch(92.3% .003 48.717);"></td></tr>
///         <tr><td>stone-300</td><td>oklch(86.9% .005 56.366)</td><td style="background-color: oklch(86.9% .005 56.366);"></td></tr>
///         <tr><td>stone-400</td><td>oklch(70.9% .01 56.259)</td><td style="background-color: oklch(70.9% .01 56.259);"></td></tr>
///         <tr><td>stone-500</td><td>oklch(55.3% .013 58.071)</td><td style="background-color: oklch(55.3% .013 58.071);"></td></tr>
///         <tr><td>stone-600</td><td>oklch(44.4% .011 73.639)</td><td style="background-color: oklch(44.4% .011 73.639);"></td></tr>
///         <tr><td>stone-700</td><td>oklch(37.4% .01 67.558)</td><td style="background-color: oklch(37.4% .01 67.558);"></td></tr>
///         <tr><td>stone-800</td><td>oklch(26.8% .007 34.298)</td><td style="background-color: oklch(26.8% .007 34.298);"></td></tr>
///         <tr><td>stone-900</td><td>oklch(21.6% .006 56.043)</td><td style="background-color: oklch(21.6% .006 56.043);"></td></tr>
///         <tr><td>stone-950</td><td>oklch(14.7% .004 49.25)</td><td style="background-color: oklch(14.7% .004 49.25);"></td></tr>
///         <tr><td>black</td><td>#000</td><td style="background-color: #000;"></td></tr>
///         <tr><td>white</td><td>#fff</td><td style="background-color: #fff;"></td></tr>
///     </tbody>
/// </table>
///
/// Based on [Tailwind's default color palette](https://tailwindcss.com/docs/customizing-colors).
pub const BUILTIN_COLORS: phf::Map<&str, &'static str> = map! {
    "red-50" => "oklch(97.1% .013 17.38)",
    "red-100" => "oklch(93.6% .032 17.717)",
    "red-200" => "oklch(88.5% .062 18.334)",
    "red-300" => "oklch(80.8% .114 19.571)",
    "red-400" => "oklch(70.4% .191 22.216)",
    "red-500" => "oklch(63.7% .237 25.331)",
    "red-600" => "oklch(57.7% .245 27.325)",
    "red-700" => "oklch(50.5% .213 27.518)",
    "red-800" => "oklch(44.4% .177 26.899)",
    "red-900" => "oklch(39.6% .141 25.723)",
    "red-950" => "oklch(25.8% .092 26.042)",
    "orange-50" => "oklch(98% .016 73.684)",
    "orange-100" => "oklch(95.4% .038 75.164)",
    "orange-200" => "oklch(90.1% .076 70.697)",
    "orange-300" => "oklch(83.7% .128 66.29)",
    "orange-400" => "oklch(75% .183 55.934)",
    "orange-500" => "oklch(70.5% .213 47.604)",
    "orange-600" => "oklch(64.6% .222 41.116)",
    "orange-700" => "oklch(55.3% .195 38.402)",
    "orange-800" => "oklch(47% .157 37.304)",
    "orange-900" => "oklch(40.8% .123 38.172)",
    "orange-950" => "oklch(26.6% .079 36.259)",
    "amber-50" => "oklch(98.7% .022 95.277)",
    "amber-100" => "oklch(96.2% .059 95.617)",
    "amber-200" => "oklch(92.4% .12 95.746)",
    "amber-300" => "oklch(87.9% .169 91.605)",
    "amber-400" => "oklch(82.8% .189 84.429)",
    "amber-500" => "oklch(76.9% .188 70.08)",
    "amber-600" => "oklch(66.6% .179 58.318)",
    "amber-700" => "oklch(55.5% .163 48.998)",
    "amber-800" => "oklch(47.3% .137 46.201)",
    "amber-900" => "oklch(41.4% .112 45.904)",
    "amber-950" => "oklch(27.9% .077 45.635)",
    "yellow-50" => "oklch(98.7% .026 102.212)",
    "yellow-100" => "oklch(97.3% .071 103.193)",
    "yellow-200" => "oklch(94.5% .129 101.54)",
    "yellow-300" => "oklch(90.5% .182 98.111)",
    "yellow-400" => "oklch(85.2% .199 91.936)",
    "yellow-500" => "oklch(79.5% .184 86.047)",
    "yellow-600" => "oklch(68.1% .162 75.834)",
    "yellow-700" => "oklch(55.4% .135 66.442)",
    "yellow-800" => "oklch(47.6% .114 61.907)",
    "yellow-900" => "oklch(42.1% .095 57.708)",
    "yellow-950" => "oklch(28.6% .066 53.813)",
    "lime-50" => "oklch(98.6% .031 120.757)",
    "lime-100" => "oklch(96.7% .067 122.328)",
    "lime-200" => "oklch(93.8% .127 124.321)",
    "lime-300" => "oklch(89.7% .196 126.665)",
    "lime-400" => "oklch(84.1% .238 128.85)",
    "lime-500" => "oklch(76.8% .233 130.85)",
    "lime-600" => "oklch(64.8% .2 131.684)",
    "lime-700" => "oklch(53.2% .157 131.589)",
    "lime-800" => "oklch(45.3% .124 130.933)",
    "lime-900" => "oklch(40.5% .101 131.063)",
    "lime-950" => "oklch(27.4% .072 132.109)",
    "green-50" => "oklch(98.2% .018 155.826)",
    "green-100" => "oklch(96.2% .044 156.743)",
    "green-200" => "oklch(92.5% .084 155.995)",
    "green-300" => "oklch(87.1% .15 154.449)",
    "green-400" => "oklch(79.2% .209 151.711)",
    "green-500" => "oklch(72.3% .219 149.579)",
    "green-600" => "oklch(62.7% .194 149.214)",
    "green-700" => "oklch(52.7% .154 150.069)",
    "green-800" => "oklch(44.8% .119 151.328)",
    "green-900" => "oklch(39.3% .095 152.535)",
    "green-950" => "oklch(26.6% .065 152.934)",
    "emerald-50" => "oklch(97.9% .021 166.113)",
    "emerald-100" => "oklch(95% .052 163.051)",
    "emerald-200" => "oklch(90.5% .093 164.15)",
    "emerald-300" => "oklch(84.5% .143 164.978)",
    "emerald-400" => "oklch(76.5% .177 163.223)",
    "emerald-500" => "oklch(69.6% .17 162.48)",
    "emerald-600" => "oklch(59.6% .145 163.225)",
    "emerald-700" => "oklch(50.8% .118 165.612)",
    "emerald-800" => "oklch(43.2% .095 166.913)",
    "emerald-900" => "oklch(37.8% .077 168.94)",
    "emerald-950" => "oklch(26.2% .051 172.552)",
    "teal-50" => "oklch(98.4% .014 180.72)",
    "teal-100" => "oklch(95.3% .051 180.801)",
    "teal-200" => "oklch(91% .096 180.426)",
    "teal-300" => "oklch(85.5% .138 181.071)",
    "teal-400" => "oklch(77.7% .152 181.912)",
    "teal-500" => "oklch(70.4% .14 182.503)",
    "teal-600" => "oklch(60% .118 184.704)",
    "teal-700" => "oklch(51.1% .096 186.391)",
    "teal-800" => "oklch(43.7% .078 188.216)",
    "teal-900" => "oklch(38.6% .063 188.416)",
    "teal-950" => "oklch(27.7% .046 192.524)",
    "cyan-50" => "oklch(98.4% .019 200.873)",
    "cyan-100" => "oklch(95.6% .045 203.388)",
    "cyan-200" => "oklch(91.7% .08 205.041)",
    "cyan-300" => "oklch(86.5% .127 207.078)",
    "cyan-400" => "oklch(78.9% .154 211.53)",
    "cyan-500" => "oklch(71.5% .143 215.221)",
    "cyan-600" => "oklch(60.9% .126 221.723)",
    "cyan-700" => "oklch(52% .105 223.128)",
    "cyan-800" => "oklch(45% .085 224.283)",
    "cyan-900" => "oklch(39.8% .07 227.392)",
    "cyan-950" => "oklch(30.2% .056 229.695)",
    "sky-50" => "oklch(97.7% .013 236.62)",
    "sky-100" => "oklch(95.1% .026 236.824)",
    "sky-200" => "oklch(90.1% .058 230.902)",
    "sky-300" => "oklch(82.8% .111 230.318)",
    "sky-400" => "oklch(74.6% .16 232.661)",
    "sky-500" => "oklch(68.5% .169 237.323)",
    "sky-600" => "oklch(58.8% .158 241.966)",
    "sky-700" => "oklch(50% .134 242.749)",
    "sky-800" => "oklch(44.3% .11 240.79)",
    "sky-900" => "oklch(39.1% .09 240.876)",
    "sky-950" => "oklch(29.3% .066 243.157)",
    "blue-50" => "oklch(97% .014 254.604)",
    "blue-100" => "oklch(93.2% .032 255.585)",
    "blue-200" => "oklch(88.2% .059 254.128)",
    "blue-300" => "oklch(80.9% .105 251.813)",
    "blue-400" => "oklch(70.7% .165 254.624)",
    "blue-500" => "oklch(62.3% .214 259.815)",
    "blue-600" => "oklch(54.6% .245 262.881)",
    "blue-700" => "oklch(48.8% .243 264.376)",
    "blue-800" => "oklch(42.4% .199 265.638)",
    "blue-900" => "oklch(37.9% .146 265.522)",
    "blue-950" => "oklch(28.2% .091 267.935)",
    "indigo-50" => "oklch(96.2% .018 272.314)",
    "indigo-100" => "oklch(93% .034 272.788)",
    "indigo-200" => "oklch(87% .065 274.039)",
    "indigo-300" => "oklch(78.5% .115 274.713)",
    "indigo-400" => "oklch(67.3% .182 276.935)",
    "indigo-500" => "oklch(58.5% .233 277.117)",
    "indigo-600" => "oklch(51.1% .262 276.966)",
    "indigo-700" => "oklch(45.7% .24 277.023)",
    "indigo-800" => "oklch(39.8% .195 277.366)",
    "indigo-900" => "oklch(35.9% .144 278.697)",
    "indigo-950" => "oklch(25.7% .09 281.288)",
    "violet-50" => "oklch(96.9% .016 293.756)",
    "violet-100" => "oklch(94.3% .029 294.588)",
    "violet-200" => "oklch(89.4% .057 293.283)",
    "violet-300" => "oklch(81.1% .111 293.571)",
    "violet-400" => "oklch(70.2% .183 293.541)",
    "violet-500" => "oklch(60.6% .25 292.717)",
    "violet-600" => "oklch(54.1% .281 293.009)",
    "violet-700" => "oklch(49.1% .27 292.581)",
    "violet-800" => "oklch(43.2% .232 292.759)",
    "violet-900" => "oklch(38% .189 293.745)",
    "violet-950" => "oklch(28.3% .141 291.089)",
    "purple-50" => "oklch(97.7% .014 308.299)",
    "purple-100" => "oklch(94.6% .033 307.174)",
    "purple-200" => "oklch(90.2% .063 306.703)",
    "purple-300" => "oklch(82.7% .119 306.383)",
    "purple-400" => "oklch(71.4% .203 305.504)",
    "purple-500" => "oklch(62.7% .265 303.9)",
    "purple-600" => "oklch(55.8% .288 302.321)",
    "purple-700" => "oklch(49.6% .265 301.924)",
    "purple-800" => "oklch(43.8% .218 303.724)",
    "purple-900" => "oklch(38.1% .176 304.987)",
    "purple-950" => "oklch(29.1% .149 302.717)",
    "fuchsia-50" => "oklch(97.7% .017 320.058)",
    "fuchsia-100" => "oklch(95.2% .037 318.852)",
    "fuchsia-200" => "oklch(90.3% .076 319.62)",
    "fuchsia-300" => "oklch(83.3% .145 321.434)",
    "fuchsia-400" => "oklch(74% .238 322.16)",
    "fuchsia-500" => "oklch(66.7% .295 322.15)",
    "fuchsia-600" => "oklch(59.1% .293 322.896)",
    "fuchsia-700" => "oklch(51.8% .253 323.949)",
    "fuchsia-800" => "oklch(45.2% .211 324.591)",
    "fuchsia-900" => "oklch(40.1% .17 325.612)",
    "fuchsia-950" => "oklch(29.3% .136 325.661)",
    "pink-50" => "oklch(97.1% .014 343.198)",
    "pink-100" => "oklch(94.8% .028 342.258)",
    "pink-200" => "oklch(89.9% .061 343.231)",
    "pink-300" => "oklch(82.3% .12 346.018)",
    "pink-400" => "oklch(71.8% .202 349.761)",
    "pink-500" => "oklch(65.6% .241 354.308)",
    "pink-600" => "oklch(59.2% .249 .584)",
    "pink-700" => "oklch(52.5% .223 3.958)",
    "pink-800" => "oklch(45.9% .187 3.815)",
    "pink-900" => "oklch(40.8% .153 2.432)",
    "pink-950" => "oklch(28.4% .109 3.907)",
    "rose-50" => "oklch(96.9% .015 12.422)",
    "rose-100" => "oklch(94.1% .03 12.58)",
    "rose-200" => "oklch(89.2% .058 10.001)",
    "rose-300" => "oklch(81% .117 11.638)",
    "rose-400" => "oklch(71.2% .194 13.428)",
    "rose-500" => "oklch(64.5% .246 16.439)",
    "rose-600" => "oklch(58.6% .253 17.585)",
    "rose-700" => "oklch(51.4% .222 16.935)",
    "rose-800" => "oklch(45.5% .188 13.697)",
    "rose-900" => "oklch(41% .159 10.272)",
    "rose-950" => "oklch(27.1% .105 12.094)",
    "slate-50" => "oklch(98.4% .003 247.858)",
    "slate-100" => "oklch(96.8% .007 247.896)",
    "slate-200" => "oklch(92.9% .013 255.508)",
    "slate-300" => "oklch(86.9% .022 252.894)",
    "slate-400" => "oklch(70.4% .04 256.788)",
    "slate-500" => "oklch(55.4% .046 257.417)",
    "slate-600" => "oklch(44.6% .043 257.281)",
    "slate-700" => "oklch(37.2% .044 257.287)",
    "slate-800" => "oklch(27.9% .041 260.031)",
    "slate-900" => "oklch(20.8% .042 265.755)",
    "slate-950" => "oklch(12.9% .042 264.695)",
    "gray-50" => "oklch(98.5% .002 247.839)",
    "gray-100" => "oklch(96.7% .003 264.542)",
    "gray-200" => "oklch(92.8% .006 264.531)",
    "gray-300" => "oklch(87.2% .01 258.338)",
    "gray-400" => "oklch(70.7% .022 261.325)",
    "gray-500" => "oklch(55.1% .027 264.364)",
    "gray-600" => "oklch(44.6% .03 256.802)",
    "gray-700" => "oklch(37.3% .034 259.733)",
    "gray-800" => "oklch(27.8% .033 256.848)",
    "gray-900" => "oklch(21% .034 264.665)",
    "gray-950" => "oklch(13% .028 261.692)",
    "zinc-50" => "oklch(98.5% 0 0)",
    "zinc-100" => "oklch(96.7% .001 286.375)",
    "zinc-200" => "oklch(92% .004 286.32)",
    "zinc-300" => "oklch(87.1% .006 286.286)",
    "zinc-400" => "oklch(70.5% .015 286.067)",
    "zinc-500" => "oklch(55.2% .016 285.938)",
    "zinc-600" => "oklch(44.2% .017 285.786)",
    "zinc-700" => "oklch(37% .013 285.805)",
    "zinc-800" => "oklch(27.4% .006 286.033)",
    "zinc-900" => "oklch(21% .006 285.885)",
    "zinc-950" => "oklch(14.1% .005 285.823)",
    "neutral-50" => "oklch(98.5% 0 0)",
    "neutral-100" => "oklch(97% 0 0)",
    "neutral-200" => "oklch(92.2% 0 0)",
    "neutral-300" => "oklch(87% 0 0)",
    "neutral-400" => "oklch(70.8% 0 0)",
    "neutral-500" => "oklch(55.6% 0 0)",
    "neutral-600" => "oklch(43.9% 0 0)",
    "neutral-700" => "oklch(37.1% 0 0)",
    "neutral-800" => "oklch(26.9% 0 0)",
    "neutral-900" => "oklch(20.5% 0 0)",
    "neutral-950" => "oklch(14.5% 0 0)",
    "stone-50" => "oklch(98.5% .001 106.423)",
    "stone-100" => "oklch(97% .001 106.424)",
    "stone-200" => "oklch(92.3% .003 48.717)",
    "stone-300" => "oklch(86.9% .005 56.366)",
    "stone-400" => "oklch(70.9% .01 56.259)",
    "stone-500" => "oklch(55.3% .013 58.071)",
    "stone-600" => "oklch(44.4% .011 73.639)",
    "stone-700" => "oklch(37.4% .01 67.558)",
    "stone-800" => "oklch(26.8% .007 34.298)",
    "stone-900" => "oklch(21.6% .006 56.043)",
    "stone-950" => "oklch(14.7% .004 49.25)",
    "black" => "#000",
    "white" => "#fff",
};

/// The list of all default screen breakpoints.
///
/// Based on [Tailwind's default screen breakpoints](https://tailwindcss.com/docs/hover-focus-and-other-states#quick-reference).
pub const BUILTIN_SCREENS: &[(&str, &str)] = &[
    ("sm", "40rem"),
    ("md", "48rem"),
    ("lg", "64rem"),
    ("xl", "80rem"),
    ("2xl", "96rem"),
];

/// The list of all default container size breakpoints.
///
/// Based on [Tailwind's default container size breakpoints](https://tailwindcss.com/docs/hover-focus-and-other-states#quick-reference).
pub const BUILTIN_CONTAINERS: &[(&str, &str)] = &[
    ("3xs", "16rem"),
    ("2xs", "18rem"),
    ("xs", "20rem"),
    ("sm", "24rem"),
    ("md", "28rem"),
    ("lg", "32rem"),
    ("xl", "36rem"),
    ("2xl", "42rem"),
    ("3xl", "48rem"),
    ("4xl", "56rem"),
    ("5xl", "64rem"),
    ("6xl", "72rem"),
    ("7xl", "80rem"),
];

/// The list of all default variants (sorted following their importance in the CSS file).
///
/// - `not-[{}]`: applies [`:not()`](https://developer.mozilla.org/en-US/docs/Web/CSS/:not) with the value specified between the square brackets
/// - `group-[{}]`: applies a selector to the group then selects all its child nodes
/// - `peer-[{}]`: applies a selector to a peer element then selects all its peer nodes
/// - `first-letter`: applies the [`first-letter`](https://developer.mozilla.org/en-US/docs/Web/CSS/::first-letter) pseudo element
/// - `first-line`: applies the [`first-line`](https://developer.mozilla.org/en-US/docs/Web/CSS/::first-line) pseudo element
/// - `marker`: applies the [`marker`](https://developer.mozilla.org/en-US/docs/Web/CSS/::marker) pseudo element to the element **and all of its nested children**
/// - `selection`: applies the [`selection`](https://developer.mozilla.org/en-US/docs/Web/CSS/::selection) pseudo element to the element **and all of its nested children**
/// - `file`: applies the [`file-selector-button`](https://developer.mozilla.org/en-US/docs/Web/CSS/::file-selector-button) pseudo element
/// - `placeholder`: applies the [`placeholder`](https://developer.mozilla.org/en-US/docs/Web/CSS/::placeholder) pseudo element
/// - `backdrop`: applies the [`backdrop`](https://developer.mozilla.org/en-US/docs/Web/CSS/::backdrop) pseudo element
/// - `details-content`: applies the [`details-content`](https://developer.mozilla.org/en-US/docs/Web/CSS/::details-content) pseudo element
/// - `before`: applies the [`before`](https://developer.mozilla.org/en-US/docs/Web/CSS/::before) pseudo element
/// - `after`: applies the [`before`](https://developer.mozilla.org/en-US/docs/Web/CSS/::before) pseudo element
/// - `all` or `**`: selects all nested children instead of the element itself
/// - `children` or `*`: selects all direct children (only one level deep) instead of the element itself
/// - `siblings`: selects all siblings of the element using the [general sibling combinator `~`](https://developer.mozilla.org/en-US/docs/Web/CSS/General_sibling_combinator)
/// - `sibling`: select the sibling next to the element using the [adjacent sibling combinator `+`](https://developer.mozilla.org/en-US/docs/Web/CSS/Adjacent_sibling_combinator)
/// - `first`: applies the [`first-child`](https://developer.mozilla.org/en-US/docs/Web/CSS/:first-child) pseudo class
/// - `not-first`: opposite of `first`, applies `:not(:first-child)`
/// - `last`: applies the [`last-child`](https://developer.mozilla.org/en-US/docs/Web/CSS/:last-child) pseudo class
/// - `not-last`: opposite of `last`, applies `:not(:last-child)`
/// - `only`: applies the [`only-child`](https://developer.mozilla.org/en-US/docs/Web/CSS/:only-child) pseudo class
/// - `not-only`: opposite of `only`, applies `:not(:only-child)`
/// - `odd`: applies the [`nth-child(odd)`](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-child) pseudo class
/// - `even`: opposite of `odd`, applies the [`nth-child(even)`](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-child) pseudo class
/// - `first-of-type`: applies the [`first-of-type`](https://developer.mozilla.org/en-US/docs/Web/CSS/:first-of-type) pseudo class
/// - `last-of-type`: applies the [`last-of-type`](https://developer.mozilla.org/en-US/docs/Web/CSS/:last-of-type) pseudo class
/// - `only-of-type`: applies the [`only-of-type`](https://developer.mozilla.org/en-US/docs/Web/CSS/:only-of-type) pseudo class
/// - `not-first-of-type`: opposite of `first-of-type`, applies `:not(:first-of-type)`
/// - `not-last-of-type`: opposite of `last-of-type`, applies `:not(:last-of-type)`
/// - `not-only-of-type`: opposite of `only-of-type`, applies `:not(:only-of-type)`
/// - `visited`: applies the [`visited`](https://developer.mozilla.org/en-US/docs/Web/CSS/:visited) pseudo class
/// - `target`: applies the [`target`](https://developer.mozilla.org/en-US/docs/Web/CSS/:target) pseudo class
/// - `open`: selects all elements having the `open` attributes, useful for [`<dialog>` elements](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog)
/// - `default`: applies the [`default`](https://developer.mozilla.org/en-US/docs/Web/CSS/:default) pseudo class
/// - `checked`: applies the [`checked`](https://developer.mozilla.org/en-US/docs/Web/CSS/:checked) pseudo class
/// - `not-checked`: opposite of `checked`, applies `:not(:checked)`
/// - `indeterminate`: applies the [`indeterminate`](https://developer.mozilla.org/en-US/docs/Web/CSS/:indeterminate) pseudo class
/// - `placeholder-shown`: applies the [`placeholder-shown`](https://developer.mozilla.org/en-US/docs/Web/CSS/:placeholder-shown) pseudo class
/// - `autofill`: applies the [`autofill`](https://developer.mozilla.org/en-US/docs/Web/CSS/:autofill) pseudo class
/// - `optional`: applies the [`optional`](https://developer.mozilla.org/en-US/docs/Web/CSS/:optional) pseudo class
/// - `required`: applies the [`required`](https://developer.mozilla.org/en-US/docs/Web/CSS/:required) pseudo class
/// - `valid`: applies the [`valid`](https://developer.mozilla.org/en-US/docs/Web/CSS/:valid) pseudo class
/// - `invalid`: applies the [`invalid`](https://developer.mozilla.org/en-US/docs/Web/CSS/:invalid) pseudo class
/// - `in-range`: applies the [`in-range`](https://developer.mozilla.org/en-US/docs/Web/CSS/:in-range) pseudo class
/// - `out-of-range`: applies the [`out-of-range`](https://developer.mozilla.org/en-US/docs/Web/CSS/:out-of-range) pseudo class
/// - `read-only`: applies the [`read-only`](https://developer.mozilla.org/en-US/docs/Web/CSS/:read-only) pseudo class
/// - `read-write`: applies the [`read-write`](https://developer.mozilla.org/en-US/docs/Web/CSS/:read-write) pseudo class
/// - `empty`: applies the [`empty`](https://developer.mozilla.org/en-US/docs/Web/CSS/:empty) pseudo class
/// - `focus-within`: applies the [`focus-within`](https://developer.mozilla.org/en-US/docs/Web/CSS/:focus-within) pseudo class
/// - `hover`: applies the [`hover`](https://developer.mozilla.org/en-US/docs/Web/CSS/:hover) pseudo class
/// - `focus`: applies the [`focus`](https://developer.mozilla.org/en-US/docs/Web/CSS/:focus) pseudo class
/// - `focus-visible`: applies the [`focus-visible`](https://developer.mozilla.org/en-US/docs/Web/CSS/:focus-visible) pseudo class
/// - `active`: applies the [`active`](https://developer.mozilla.org/en-US/docs/Web/CSS/:active) pseudo class
/// - `enabled`: applies the [`enabled`](https://developer.mozilla.org/en-US/docs/Web/CSS/:enabled) pseudo class
/// - `disabled`: applies the [`disabled`](https://developer.mozilla.org/en-US/docs/Web/CSS/:disabled) pseudo class
/// - `inert`: selects all the elements (and their child nodes) having the [`inert` attribute](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/inert)
/// - `in-[{}]`: selects the element based on the state of the parent elements
/// - `has-[{}]`: selects elements using [`:has()`](https://developer.mozilla.org/en-US/docs/Web/CSS/:has)
/// - `aria-busy`: selects all elements having the [`aria-busy="true"` attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-busy)
/// - `aria-checked`: selects all elements having the [`aria-checked="true"` attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-checked)
/// - `aria-disabled`: selects all elements having the [`aria-disabled="true"` attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-disabled)
/// - `aria-expanded`: selects all elements having the [`aria-expanded="true"` attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded)
/// - `aria-hidden`: selects all elements having the [`aria-hidden="true"` attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden)
/// - `aria-pressed`: selects all elements having the [`aria-pressed="true"` attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-pressed)
/// - `aria-readonly`: selects all elements having the [`aria-readonly="true"` attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-readonly)
/// - `aria-required`: selects all elements having the [`aria-required="true"` attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-required)
/// - `aria-selected`: selects all elements having the [`aria-selected="true"` attribute](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-selected)
/// - `aria-[{}]`: selects all elements having a custom aria attribute
/// - `data-[{}]`: selects all elements having the [`data-{}` attribute](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/data-*)
/// - `nth-[{}]`: selects all elements having a specific position in a list of children using the [`:nth-child({})` pseudo class](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-child)
/// - `nth-last-[{}]`: selects all elements having a specific position in a list of children using the [`:nth-last-child({})` pseudo class](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-last-child)
/// - `nth-of-type-[{}]`: selects all elements having a specific type using the [`:nth-of-type-({})` pseudo class](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-of-type)
/// - `nth-last-of-type-[{}]`: selects all elements having a specific type using the [`:nth-last-of-type-({})` pseudo class](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-of-type)
/// - `supports-[{}]`: applies the [`@supports`](https://developer.mozilla.org/en-US/docs/Web/CSS/@supports) at-rule
/// - `motion-safe`: applies the [`@media (prefers-reduced-motion: no-preference)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion) at-rule
/// - `motion-reduce`: applies the [`@media (prefers-reduced-motion: reduce)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion) at-rule
/// - `contrast-more`: applies the [`@media (prefers-contrast: more)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-contrast) at-rule
/// - `contrast-less`: applies the [`@media (prefers-contrast: less)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-contrast) at-rule
/// - `max`: applies the [`@media (width < {})`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/width) at-rule
/// - `min`: applies the [`@media (width >= {})`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/width) at-rule
/// - `@max`: applies the [`@container (width < {})`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/width) at-rule
/// - `@min`: applies the [`@container (width >= {})`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/width) at-rule
/// - `portrait`: applies the [`@media (orientation: portrait)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/orientation) at-rule
/// - `landscape`: applies the [`@media (orientation: landscape)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/orientation) at-rule
/// - `ltr`: selects all elements contained in an element having the [`dir="ltr"` attribute](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/dir), useful when styling based on writing direction
/// - `rtl`: selects all elements contained in an element having the [`dir="rtl"` attribute](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/dir), useful when styling based on writing direction
/// - `starting`: applies the [`@starting-style`](https://developer.mozilla.org/en-US/docs/Web/CSS/@starting-style) at-rule
/// - `print`: applies the [`@media print`](https://developer.mozilla.org/en-US/docs/Web/Guide/Printing#using_media_queries_to_improve_layout) at-rule
/// - `forced-colors`: applies the [`@media (forced-colors: active)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/forced-colors) at-rule
/// - `inverted-colors`: applies the [`@media (inverted-colors: inverted)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/inverted-colors) at-rule
/// - `pointer-none`: applies the [`@media (pointer: none)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/pointer) at-rule
/// - `pointer-coarse`: applies the [`@media (pointer: coarse)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/pointer) at-rule
/// - `pointer-fine`: applies the [`@media (pointer: fine)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/pointer) at-rule
/// - `any-pointer-none`: applies the [`@media (any-pointer: none)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/any-pointer) at-rule
/// - `any-pointer-coarse`: applies the [`@media (any-pointer: coarse)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/any-pointer) at-rule
/// - `any-pointer-fine`: applies the [`@media (any-pointer: fine)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/any-pointer) at-rule
/// - `noscript`: applies the [`@media (scripting: none)`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/scripting) at-rule
///
/// Besides the default variants, some others are auto-generated from the configuration like
/// breakpoints (see [`BUILTIN_SCREENS`]), container sizes (see [`BUILTIN_CONTAINERS`]) and the dark mode (`dark:` variant).
///
/// You can prefix all variants by `group-`, `peer-` or `peer-not-` so that they apply to the group
/// or peer element, e.g `group-hover:bg-blue-100` or `peer-has-[#test]:hidden`.
///
/// Based on [Tailwind's default variants](https://tailwindcss.com/docs/hover-focus-and-other-states).
#[rustfmt::skip]
pub const BUILTIN_VARIANTS: phf::OrderedMap<&'static str, Variant> = {
    let mut counter = 0;

    phf_ordered_map! {
        "not" => Variant::new_const(&mut counter, "&:not({})").with_prefixed(),

        "first-letter" => Variant::new_const(&mut counter, "&::first-letter"),
        "first-line" => Variant::new_const(&mut counter, "&::first-line"),
        "marker" => Variant::new_const(&mut counter, "& *::marker, &::marker"),
        "selection" => Variant::new_const(&mut counter, "& *::selection, &::selection"),
        "file" => Variant::new_const(&mut counter, "&::file-selector-button, &::-webkit-file-upload-button"),
        "placeholder" => Variant::new_const(&mut counter, "&::placeholder"),
        "backdrop" => Variant::new_const(&mut counter, "&::backdrop"),
        "details-content" => Variant::new_const(&mut counter, "&::details-content"),
        "before" => Variant::new_const(&mut counter, "&::before"),
        "after" => Variant::new_const(&mut counter, "&::after"),
        "all" => Variant::new_const(&mut counter, "& *"),
        "**" => Variant::new_const(&mut counter, "& *"),
        "children" => Variant::new_const(&mut counter, "& > *"),
        "*" => Variant::new_const(&mut counter, "& > *"),
        "siblings" => Variant::new_const(&mut counter, "& ~ *"),
        "sibling" => Variant::new_const(&mut counter, "& + *"),

        "first" => Variant::new_const(&mut counter, "&:first-child"),
        "not-first" => Variant::new_const(&mut counter, "&:not(:first-child)"),
        "last" => Variant::new_const(&mut counter, "&:last-child"),
        "not-last" => Variant::new_const(&mut counter, "&:not(:last-child)"),
        "only" => Variant::new_const(&mut counter, "&:only-child"),
        "not-only" => Variant::new_const(&mut counter, "&:not(:only-child)"),
        "odd" => Variant::new_const(&mut counter, "&:nth-child(odd)"),
        "even" => Variant::new_const(&mut counter, "&:nth-child(even)"),
        "first-of-type" => Variant::new_const(&mut counter, "&:first-of-type"),
        "last-of-type" => Variant::new_const(&mut counter, "&:last-of-type"),
        "only-of-type" => Variant::new_const(&mut counter, "&:only-of-type"),
        "not-first-of-type" => Variant::new_const(&mut counter, "&:not(:first-of-type)"),
        "not-last-of-type" => Variant::new_const(&mut counter, "&:not(:last-of-type)"),
        "not-only-of-type" => Variant::new_const(&mut counter, "&:not(:only-of-type)"),
        "visited" => Variant::new_const(&mut counter, "&:visited"),
        "target" => Variant::new_const(&mut counter, "&:target"),
        "open" => Variant::new_const(&mut counter, "&[open]"),
        "default" => Variant::new_const(&mut counter, "&:default"),
        "checked" => Variant::new_const(&mut counter, "&:checked"),
        "not-checked" => Variant::new_const(&mut counter, "&:not(:checked)"),
        "indeterminate" => Variant::new_const(&mut counter, "&:indeterminate"),
        "placeholder-shown" => Variant::new_const(&mut counter, "&:placeholder-shown"),
        "autofill" => Variant::new_const(&mut counter, "&:autofill"),
        "optional" => Variant::new_const(&mut counter, "&:optional"),
        "required" => Variant::new_const(&mut counter, "&:required"),
        "valid" => Variant::new_const(&mut counter, "&:valid"),
        "invalid" => Variant::new_const(&mut counter, "&:invalid"),
        "in-range" => Variant::new_const(&mut counter, "&:in-range"),
        "out-of-range" => Variant::new_const(&mut counter, "&:out-of-range"),
        "read-only" => Variant::new_const(&mut counter, "&:read-only"),
        "read-write" => Variant::new_const(&mut counter, "&:read-write"),
        "empty" => Variant::new_const(&mut counter, "&:empty"),
        "focus-within" => Variant::new_const(&mut counter, "&:focus-within"),
        "hover" => Variant::new_const(&mut counter, "&:hover"),
        "focus" => Variant::new_const(&mut counter, "&:focus"),
        "focus-visible" => Variant::new_const(&mut counter, "&:focus-visible"),
        "active" => Variant::new_const(&mut counter, "&:active"),
        "enabled" => Variant::new_const(&mut counter, "&:enabled"),
        "disabled" => Variant::new_const(&mut counter, "&:disabled"),
        "inert" => Variant::new_const(&mut counter, "&:is([inert], [inert] *)"),
        "in" => Variant::new_const(&mut counter, ":where({}) &").with_prefixed(),
        "has" => Variant::new_const(&mut counter, "&:has({})").with_prefixed(),

        "aria-busy" => Variant::new_const(&mut counter, "&[aria-busy=\"true\"]"),
        "aria-checked" => Variant::new_const(&mut counter, "&[aria-checked=\"true\"]"),
        "aria-disabled" => Variant::new_const(&mut counter, "&[aria-disabled=\"true\"]"),
        "aria-expanded" => Variant::new_const(&mut counter, "&[aria-expanded=\"true\"]"),
        "aria-hidden" => Variant::new_const(&mut counter, "&[aria-hidden=\"true\"]"),
        "aria-pressed" => Variant::new_const(&mut counter, "&[aria-pressed=\"true\"]"),
        "aria-readonly" => Variant::new_const(&mut counter, "&[aria-readonly=\"true\"]"),
        "aria-required" => Variant::new_const(&mut counter, "&[aria-required=\"true\"]"),
        "aria-selected" => Variant::new_const(&mut counter, "&[aria-selected=\"true\"]"),
        "aria" => Variant::new_const(&mut counter, "&[aria-{}]").with_prefixed(),

        "data" => Variant::new_const(&mut counter, "&[data-{}]").with_prefixed(),
        "nth" => Variant::new_const(&mut counter, "&:nth-child({})").with_prefixed(),
        "nth-last" => Variant::new_const(&mut counter, "&:nth-last-child({})").with_prefixed(),
        "nth-of-type" => Variant::new_const(&mut counter, "&:nth-of-type({})").with_prefixed(),
        "nth-last-of-type" => Variant::new_const(&mut counter, "&:nth-last-of-type({})").with_prefixed(),
        "supports" => Variant::new_const(&mut counter, "@supports ({})").with_prefixed(),

        "motion-safe" => Variant::new_const(&mut counter, "@media (prefers-reduced-motion: no-preference)"),
        "motion-reduce" => Variant::new_const(&mut counter, "@media (prefers-reduced-motion: reduce)"),
        "contrast-more" => Variant::new_const(&mut counter, "@media (prefers-contrast: more)"),
        "contrast-less" => Variant::new_const(&mut counter, "@media (prefers-contrast: less)"),

        "max" => Variant::new_const(&mut counter, "@media (width < {})").with_prefixed(),
        "min" => Variant::new_const(&mut counter, "@media (width >= {})").with_prefixed(),
        "@max" => Variant::new_const(&mut counter, "@container (width < {})").with_prefixed(),
        "@min" => Variant::new_const(&mut counter, "@container (width >= {})").with_prefixed(),

        "portrait" => Variant::new_const(&mut counter, "@media (orientation: portrait)"),
        "landscape" => Variant::new_const(&mut counter, "@media (orientation: landscape)"),
        "ltr" => Variant::new_const(&mut counter, "[dir=\"ltr\"] &"),
        "rtl" => Variant::new_const(&mut counter, "[dir=\"rtl\"] &"),
        "starting" => Variant::new_const(&mut counter, "@starting-style"),
        "print" => Variant::new_const(&mut counter, "@media print"),
        "forced-colors" => Variant::new_const(&mut counter, "@media (forced-colors: active)"),
        "inverted-colors" => Variant::new_const(&mut counter, "@media (inverted-colors: inverted)"),
        "pointer-none" => Variant::new_const(&mut counter, "@media (pointer: none)"),
        "pointer-coarse" => Variant::new_const(&mut counter, "@media (pointer: coarse)"),
        "pointer-fine" => Variant::new_const(&mut counter, "@media (pointer: fine)"),
        "any-pointer-none" => Variant::new_const(&mut counter, "@media (any-pointer: none)"),
        "any-pointer-coarse" => Variant::new_const(&mut counter, "@media (any-pointer: coarse)"),
        "any-pointer-fine" => Variant::new_const(&mut counter, "@media (any-pointer: fine)"),
        "noscript" => Variant::new_const(&mut counter, "@media (scripting: none)"),
    }
};

/// The list of all default plugins.
///
/// Sorted following [Tailwind's order](https://github.com/tailwindlabs/tailwindcss/blob/master/src/corePlugins.js).
pub const BUILTIN_PLUGINS: &[&StaticPlugin] = &[
    &layout::container::PLUGIN,
    &accessibility::screen_reader::PLUGIN,
    &interactivity::pointer_events::PLUGIN,
    &layout::visibility::PLUGIN,
    &layout::position::PLUGIN,
    &layout::placement::PLUGIN.0,
    &layout::placement::PLUGIN.1,
    &layout::placement::PLUGIN_X.0,
    &layout::placement::PLUGIN_X.1,
    &layout::placement::PLUGIN_Y.0,
    &layout::placement::PLUGIN_Y.1,
    &layout::placement::PLUGIN_START.0,
    &layout::placement::PLUGIN_START.1,
    &layout::placement::PLUGIN_END.0,
    &layout::placement::PLUGIN_END.1,
    &layout::placement::PLUGIN_TOP.0,
    &layout::placement::PLUGIN_TOP.1,
    &layout::placement::PLUGIN_RIGHT.0,
    &layout::placement::PLUGIN_RIGHT.1,
    &layout::placement::PLUGIN_BOTTOM.0,
    &layout::placement::PLUGIN_BOTTOM.1,
    &layout::placement::PLUGIN_LEFT.0,
    &layout::placement::PLUGIN_LEFT.1,
    &layout::isolation::PLUGIN,
    &layout::z_index::PLUGIN,
    &flexbox::order::PLUGIN_LIST,
    &flexbox::order::PLUGIN_NUM,
    &grid::grid_column::PLUGIN,
    &grid::grid_column::PLUGIN_ARBITRARY,
    &grid::grid_column::PLUGIN_SPAN_1,
    &grid::grid_column::PLUGIN_SPAN_2,
    &grid::grid_column::PLUGIN_START_1,
    &grid::grid_column::PLUGIN_START_2,
    &grid::grid_column::PLUGIN_END_1,
    &grid::grid_column::PLUGIN_END_2,
    &grid::grid_row::PLUGIN,
    &grid::grid_row::PLUGIN_ARBITRARY,
    &grid::grid_row::PLUGIN_SPAN_1,
    &grid::grid_row::PLUGIN_SPAN_2,
    &grid::grid_row::PLUGIN_START_1,
    &grid::grid_row::PLUGIN_START_2,
    &grid::grid_row::PLUGIN_END_1,
    &grid::grid_row::PLUGIN_END_2,
    &layout::floats::PLUGIN,
    &layout::clear::PLUGIN,
    &spacing::margin::PLUGIN.0,
    &spacing::margin::PLUGIN.1,
    &spacing::margin::PLUGIN_X.0,
    &spacing::margin::PLUGIN_X.1,
    &spacing::margin::PLUGIN_Y.0,
    &spacing::margin::PLUGIN_Y.1,
    &spacing::margin::PLUGIN_START.0,
    &spacing::margin::PLUGIN_START.1,
    &spacing::margin::PLUGIN_END.0,
    &spacing::margin::PLUGIN_END.1,
    &spacing::margin::PLUGIN_TOP.0,
    &spacing::margin::PLUGIN_TOP.1,
    &spacing::margin::PLUGIN_RIGHT.0,
    &spacing::margin::PLUGIN_RIGHT.1,
    &spacing::margin::PLUGIN_BOTTOM.0,
    &spacing::margin::PLUGIN_BOTTOM.1,
    &spacing::margin::PLUGIN_LEFT.0,
    &spacing::margin::PLUGIN_LEFT.1,
    &layout::box_sizing::PLUGIN,
    &layout::display::PLUGIN,
    &layout::aspect_ratio::PLUGIN,
    &layout::aspect_ratio::PLUGIN_ARBITRARY,
    &sizing::height::PLUGIN_SPACING,
    &sizing::height::PLUGIN_LIST,
    &sizing::height::PLUGIN_ARBITRARY,
    &sizing::max_height::PLUGIN_SPACING,
    &sizing::max_height::PLUGIN_LIST,
    &sizing::max_height::PLUGIN_ARBITRARY,
    &sizing::min_height::PLUGIN_SPACING,
    &sizing::min_height::PLUGIN_LIST,
    &sizing::min_height::PLUGIN_ARBITRARY,
    &sizing::width::PLUGIN_SPACING,
    &sizing::width::PLUGIN_LIST,
    &sizing::width::PLUGIN_ARBITRARY,
    &sizing::min_width::PLUGIN_SPACING,
    &sizing::min_width::PLUGIN_LIST,
    &sizing::min_width::PLUGIN_ARBITRARY,
    &sizing::max_width::PLUGIN_SPACING,
    &sizing::max_width::PLUGIN_LIST_1,
    &sizing::max_width::PLUGIN_LIST_2,
    &sizing::max_width::PLUGIN_ARBITRARY,
    &flexbox::flex::PLUGIN,
    &flexbox::flex::PLUGIN_ARBITRARY,
    &flexbox::flex_shrink::PLUGIN,
    &flexbox::flex_grow::PLUGIN,
    &flexbox::flex_basis::PLUGIN,
    &flexbox::flex_basis::PLUGIN_ARBITRARY,
    &table::table_layout::PLUGIN,
    &table::caption_side::PLUGIN,
    &table::border_collapse::PLUGIN,
    &table::border_spacing::PLUGIN.0,
    &table::border_spacing::PLUGIN.1,
    &table::border_spacing::PLUGIN_X.0,
    &table::border_spacing::PLUGIN_X.1,
    &table::border_spacing::PLUGIN_Y.0,
    &table::border_spacing::PLUGIN_Y.1,
    &transform::transform_origin::PLUGIN,
    &transform::transform_origin::PLUGIN_ARBITRARY,
    &transform::perspective_origin::PLUGIN,
    &transform::perspective_origin::PLUGIN_ARBITRARY,
    &transform::perspective::PLUGIN,
    &transform::perspective::PLUGIN_ARBITRARY,
    &transform::translate::PLUGIN_X.0,
    &transform::translate::PLUGIN_X.1,
    &transform::translate::PLUGIN_Y.0,
    &transform::translate::PLUGIN_Y.1,
    &transform::translate::PLUGIN_Z.0,
    &transform::translate::PLUGIN_Z.1,
    &transform::rotate::PLUGIN.0,
    &transform::rotate::PLUGIN.1,
    &transform::rotate::PLUGIN_X.0,
    &transform::rotate::PLUGIN_X.1,
    &transform::rotate::PLUGIN_Y.0,
    &transform::rotate::PLUGIN_Y.1,
    &transform::rotate::PLUGIN_Z.0,
    &transform::rotate::PLUGIN_Z.1,
    &transform::skew::PLUGIN_X.0,
    &transform::skew::PLUGIN_X.1,
    &transform::skew::PLUGIN_Y.0,
    &transform::skew::PLUGIN_Y.1,
    &transform::scale::PLUGIN.0,
    &transform::scale::PLUGIN.1,
    &transform::scale::PLUGIN_X.0,
    &transform::scale::PLUGIN_X.1,
    &transform::scale::PLUGIN_Y.0,
    &transform::scale::PLUGIN_Y.1,
    &transform::scale::PLUGIN_Z.0,
    &transform::scale::PLUGIN_Z.1,
    &transform::transform_type::PLUGIN,
    &transition::animation::PLUGIN,
    &transition::animation::PLUGIN_ARBITRARY,
    &interactivity::cursor::PLUGIN,
    &interactivity::cursor::PLUGIN_ARBITRARY,
    &interactivity::touch_action::PLUGIN,
    &interactivity::user_select::PLUGIN,
    &interactivity::resize::PLUGIN,
    &interactivity::scroll_snap_type::PLUGIN,
    &interactivity::scroll_snap_align::PLUGIN,
    &interactivity::scroll_snap_stop::PLUGIN,
    &interactivity::scroll_margin::PLUGIN.0,
    &interactivity::scroll_margin::PLUGIN.1,
    &interactivity::scroll_margin::PLUGIN_X.0,
    &interactivity::scroll_margin::PLUGIN_X.1,
    &interactivity::scroll_margin::PLUGIN_Y.0,
    &interactivity::scroll_margin::PLUGIN_Y.1,
    &interactivity::scroll_margin::PLUGIN_START.0,
    &interactivity::scroll_margin::PLUGIN_START.1,
    &interactivity::scroll_margin::PLUGIN_END.0,
    &interactivity::scroll_margin::PLUGIN_END.1,
    &interactivity::scroll_margin::PLUGIN_TOP.0,
    &interactivity::scroll_margin::PLUGIN_TOP.1,
    &interactivity::scroll_margin::PLUGIN_RIGHT.0,
    &interactivity::scroll_margin::PLUGIN_RIGHT.1,
    &interactivity::scroll_margin::PLUGIN_BOTTOM.0,
    &interactivity::scroll_margin::PLUGIN_BOTTOM.1,
    &interactivity::scroll_margin::PLUGIN_LEFT.0,
    &interactivity::scroll_margin::PLUGIN_LEFT.1,
    &interactivity::scroll_padding::PLUGIN.0,
    &interactivity::scroll_padding::PLUGIN.1,
    &interactivity::scroll_padding::PLUGIN_X.0,
    &interactivity::scroll_padding::PLUGIN_X.1,
    &interactivity::scroll_padding::PLUGIN_Y.0,
    &interactivity::scroll_padding::PLUGIN_Y.1,
    &interactivity::scroll_padding::PLUGIN_START.0,
    &interactivity::scroll_padding::PLUGIN_START.1,
    &interactivity::scroll_padding::PLUGIN_END.0,
    &interactivity::scroll_padding::PLUGIN_END.1,
    &interactivity::scroll_padding::PLUGIN_TOP.0,
    &interactivity::scroll_padding::PLUGIN_TOP.1,
    &interactivity::scroll_padding::PLUGIN_RIGHT.0,
    &interactivity::scroll_padding::PLUGIN_RIGHT.1,
    &interactivity::scroll_padding::PLUGIN_BOTTOM.0,
    &interactivity::scroll_padding::PLUGIN_BOTTOM.1,
    &interactivity::scroll_padding::PLUGIN_LEFT.0,
    &interactivity::scroll_padding::PLUGIN_LEFT.1,
    &typography::list_style_position::PLUGIN,
    &typography::list_style_type::PLUGIN,
    &typography::list_style_type::PLUGIN_ARBITRARY,
    &interactivity::appearance::PLUGIN,
    &layout::columns::PLUGIN,
    &layout::break_before::PLUGIN,
    &layout::break_inside::PLUGIN,
    &layout::break_after::PLUGIN,
    &grid::grid_auto_columns::PLUGIN,
    &grid::grid_auto_columns::PLUGIN_ARBITRARY,
    &grid::grid_auto_flow::PLUGIN,
    &grid::grid_auto_rows::PLUGIN,
    &grid::grid_auto_rows::PLUGIN_ARBITRARY,
    &grid::grid_template_columns::PLUGIN_NUMBER,
    &grid::grid_template_columns::PLUGIN_LIST,
    &grid::grid_template_columns::PLUGIN_ARBITRARY,
    &grid::grid_template_rows::PLUGIN_NUMBER,
    &grid::grid_template_rows::PLUGIN_LIST,
    &grid::grid_template_rows::PLUGIN_ARBITRARY,
    &flexbox::flex_direction::PLUGIN,
    &flexbox::flex_wrap::PLUGIN,
    &flexbox::place_content::PLUGIN,
    &flexbox::place_items::PLUGIN,
    &flexbox::align_content::PLUGIN,
    &flexbox::align_items::PLUGIN,
    &flexbox::justify_content::PLUGIN,
    &flexbox::justify_items::PLUGIN,
    &grid::gap::PLUGIN.0,
    &grid::gap::PLUGIN.1,
    &grid::gap::PLUGIN_X.0,
    &grid::gap::PLUGIN_X.1,
    &grid::gap::PLUGIN_Y.0,
    &grid::gap::PLUGIN_Y.1,
    &spacing::space_between::PLUGIN_X_1,
    &spacing::space_between::PLUGIN_X_2,
    &spacing::space_between::PLUGIN_X_3,
    &spacing::space_between::PLUGIN_Y_1,
    &spacing::space_between::PLUGIN_Y_2,
    &spacing::space_between::PLUGIN_Y_3,
    &border::divide_width::PLUGIN_X_1,
    &border::divide_width::PLUGIN_X_2,
    &border::divide_width::PLUGIN_X_3,
    &border::divide_width::PLUGIN_Y_1,
    &border::divide_width::PLUGIN_Y_2,
    &border::divide_width::PLUGIN_Y_3,
    &border::divide_style::PLUGIN,
    &border::divide_color::PLUGIN,
    &border::divide_color::PLUGIN_ARBITRARY,
    &flexbox::place_self::PLUGIN,
    &flexbox::align_self::PLUGIN,
    &flexbox::justify_self::PLUGIN,
    &layout::overflow::PLUGIN,
    &layout::overscroll_behavior::PLUGIN,
    &interactivity::scroll_behavior::PLUGIN,
    &typography::text_overflow::PLUGIN,
    &typography::whitespace::PLUGIN,
    &typography::text_wrap::PLUGIN,
    &typography::word_break::PLUGIN,
    &border::border_radius::PLUGIN.0,
    &border::border_radius::PLUGIN.1,
    &border::border_radius::PLUGIN_START.0,
    &border::border_radius::PLUGIN_START.1,
    &border::border_radius::PLUGIN_END.0,
    &border::border_radius::PLUGIN_END.1,
    &border::border_radius::PLUGIN_TOP.0,
    &border::border_radius::PLUGIN_TOP.1,
    &border::border_radius::PLUGIN_RIGHT.0,
    &border::border_radius::PLUGIN_RIGHT.1,
    &border::border_radius::PLUGIN_BOTTOM.0,
    &border::border_radius::PLUGIN_BOTTOM.1,
    &border::border_radius::PLUGIN_LEFT.0,
    &border::border_radius::PLUGIN_LEFT.1,
    &border::border_radius::PLUGIN_START_START.0,
    &border::border_radius::PLUGIN_START_START.1,
    &border::border_radius::PLUGIN_START_END.0,
    &border::border_radius::PLUGIN_START_END.1,
    &border::border_radius::PLUGIN_END_END.0,
    &border::border_radius::PLUGIN_END_END.1,
    &border::border_radius::PLUGIN_END_START.0,
    &border::border_radius::PLUGIN_END_START.1,
    &border::border_radius::PLUGIN_TOP_RIGHT.0,
    &border::border_radius::PLUGIN_TOP_RIGHT.1,
    &border::border_radius::PLUGIN_TOP_LEFT.0,
    &border::border_radius::PLUGIN_TOP_LEFT.1,
    &border::border_radius::PLUGIN_BOTTOM_RIGHT.0,
    &border::border_radius::PLUGIN_BOTTOM_RIGHT.1,
    &border::border_radius::PLUGIN_BOTTOM_LEFT.0,
    &border::border_radius::PLUGIN_BOTTOM_LEFT.1,
    &border::border_width::PLUGIN.0,
    &border::border_width::PLUGIN.1,
    &border::border_width::PLUGIN_X.0,
    &border::border_width::PLUGIN_X.1,
    &border::border_width::PLUGIN_Y.0,
    &border::border_width::PLUGIN_Y.1,
    &border::border_width::PLUGIN_START.0,
    &border::border_width::PLUGIN_START.1,
    &border::border_width::PLUGIN_END.0,
    &border::border_width::PLUGIN_END.1,
    &border::border_width::PLUGIN_TOP.0,
    &border::border_width::PLUGIN_TOP.1,
    &border::border_width::PLUGIN_RIGHT.0,
    &border::border_width::PLUGIN_RIGHT.1,
    &border::border_width::PLUGIN_BOTTOM.0,
    &border::border_width::PLUGIN_BOTTOM.1,
    &border::border_width::PLUGIN_LEFT.0,
    &border::border_width::PLUGIN_LEFT.1,
    &border::border_style::PLUGIN,
    &border::border_style::PLUGIN_ARBITRARY,
    &border::border_color::PLUGIN.0,
    &border::border_color::PLUGIN.1,
    &border::border_color::PLUGIN_X.0,
    &border::border_color::PLUGIN_X.1,
    &border::border_color::PLUGIN_Y.0,
    &border::border_color::PLUGIN_Y.1,
    &border::border_color::PLUGIN_START.0,
    &border::border_color::PLUGIN_START.1,
    &border::border_color::PLUGIN_END.0,
    &border::border_color::PLUGIN_END.1,
    &border::border_color::PLUGIN_TOP.0,
    &border::border_color::PLUGIN_TOP.1,
    &border::border_color::PLUGIN_RIGHT.0,
    &border::border_color::PLUGIN_RIGHT.1,
    &border::border_color::PLUGIN_BOTTOM.0,
    &border::border_color::PLUGIN_BOTTOM.1,
    &border::border_color::PLUGIN_LEFT.0,
    &border::border_color::PLUGIN_LEFT.1,
    &background::background_color::PLUGIN,
    &background::background_color::PLUGIN_ARBITRARY,
    &background::background_image::PLUGIN,
    &background::background_image::PLUGIN_ARBITRARY,
    &background::background_image::PLUGIN_LINEAR_1,
    &background::background_image::PLUGIN_LINEAR_2,
    &background::background_image::PLUGIN_LINEAR_3,
    &background::background_image::PLUGIN_RADIAL_1,
    &background::background_image::PLUGIN_RADIAL_2,
    &background::background_image::PLUGIN_CONIC_1,
    &background::background_image::PLUGIN_CONIC_2,
    &background::background_image::PLUGIN_CONIC_3,
    &background::gradient_color_stops::PLUGIN_FROM_1,
    &background::gradient_color_stops::PLUGIN_FROM_2,
    &background::gradient_color_stops::PLUGIN_VIA_1,
    &background::gradient_color_stops::PLUGIN_VIA_2,
    &background::gradient_color_stops::PLUGIN_TO_1,
    &background::gradient_color_stops::PLUGIN_TO_2,
    &layout::box_decoration_break::PLUGIN,
    &background::background_size::PLUGIN,
    &background::background_size::PLUGIN_ARBITRARY,
    &background::background_attachment::PLUGIN,
    &background::background_clip::PLUGIN,
    &background::background_position::PLUGIN,
    &background::background_position::PLUGIN_ARBITRARY,
    &background::background_repeat::PLUGIN,
    &background::background_origin::PLUGIN,
    &svg::fill::PLUGIN,
    &svg::fill::PLUGIN_ARBITRARY,
    &svg::stroke::PLUGIN,
    &svg::stroke::PLUGIN_ARBITRARY,
    &svg::stroke_width::PLUGIN,
    &svg::stroke_width::PLUGIN_ARBITRARY,
    &layout::object_fit::PLUGIN,
    &layout::object_position::PLUGIN,
    &layout::object_position::PLUGIN_ARBITRARY,
    &spacing::padding::PLUGIN.0,
    &spacing::padding::PLUGIN.1,
    &spacing::padding::PLUGIN_X.0,
    &spacing::padding::PLUGIN_X.1,
    &spacing::padding::PLUGIN_Y.0,
    &spacing::padding::PLUGIN_Y.1,
    &spacing::padding::PLUGIN_START.0,
    &spacing::padding::PLUGIN_START.1,
    &spacing::padding::PLUGIN_END.0,
    &spacing::padding::PLUGIN_END.1,
    &spacing::padding::PLUGIN_TOP.0,
    &spacing::padding::PLUGIN_TOP.1,
    &spacing::padding::PLUGIN_RIGHT.0,
    &spacing::padding::PLUGIN_RIGHT.1,
    &spacing::padding::PLUGIN_BOTTOM.0,
    &spacing::padding::PLUGIN_BOTTOM.1,
    &spacing::padding::PLUGIN_LEFT.0,
    &spacing::padding::PLUGIN_LEFT.1,
    &typography::text_align::PLUGIN,
    &typography::text_indent::PLUGIN,
    &typography::text_indent::PLUGIN_ARBITRARY,
    &typography::vertical_align::PLUGIN,
    &typography::font_family::PLUGIN,
    &typography::font_family::PLUGIN_ARBITRARY,
    &typography::font_size::PLUGIN,
    &typography::font_size::PLUGIN_ARBITRARY,
    &typography::font_weight::PLUGIN,
    &typography::font_weight::PLUGIN_ARBITRARY,
    &typography::text_transform::PLUGIN,
    &typography::font_style::PLUGIN,
    &typography::font_variant_numeric::PLUGIN,
    &typography::letter_spacing::PLUGIN,
    &typography::letter_spacing::PLUGIN_ARBITRARY,
    &typography::line_height::PLUGIN_LIST,
    &typography::line_height::PLUGIN_SPACING,
    &typography::line_height::PLUGIN_ARBITRARY,
    &typography::text_color::PLUGIN,
    &typography::text_color::PLUGIN_ARBITRARY,
    &typography::text_decoration::PLUGIN,
    &typography::text_decoration_color::PLUGIN,
    &typography::text_decoration_color::PLUGIN_ARBITRARY,
    &typography::text_decoration_style::PLUGIN,
    &typography::text_decoration_thickness::PLUGIN_LIST,
    &typography::text_decoration_thickness::PLUGIN_NUMBER,
    &typography::text_decoration_thickness::PLUGIN_ARBITRARY,
    &typography::text_underline_offset::PLUGIN_LIST,
    &typography::text_underline_offset::PLUGIN_NUMBER,
    &typography::text_underline_offset::PLUGIN_ARBITRARY,
    &typography::font_smoothing::PLUGIN,
    &interactivity::caret_color::PLUGIN,
    &interactivity::caret_color::PLUGIN_ARBITRARY,
    &interactivity::accent_color::PLUGIN,
    &interactivity::accent_color::PLUGIN_ARBITRARY,
    &effect::opacity::PLUGIN,
    &effect::background_blend_mode::PLUGIN,
    &effect::mix_blend_mode::PLUGIN,
    &effect::text_shadow::PLUGIN,
    &effect::text_shadow::PLUGIN_ARBITRARY,
    &effect::text_shadow_color::PLUGIN,
    &effect::text_shadow_color::PLUGIN_ARBITRARY,
    &effect::box_shadow::PLUGIN,
    &effect::box_shadow::PLUGIN_ARBITRARY,
    &effect::box_shadow_color::PLUGIN,
    &effect::box_shadow_color::PLUGIN_ARBITRARY,
    &effect::box_shadow::PLUGIN_INSET_1,
    &effect::box_shadow::PLUGIN_INSET_2,
    &effect::box_shadow_color::PLUGIN_INSET_1,
    &effect::box_shadow_color::PLUGIN_INSET_2,
    &border::outline_style::PLUGIN_LIST_1,
    &border::outline_style::PLUGIN_LIST_2,
    &border::outline_width::PLUGIN,
    &border::outline_width::PLUGIN_ARBITRARY,
    &border::outline_offset::PLUGIN,
    &border::outline_offset::PLUGIN_ARBITRARY,
    &border::outline_color::PLUGIN,
    &border::outline_color::PLUGIN_ARBITRARY,
    &border::ring_width::PLUGIN,
    &border::ring_width::PLUGIN_ARBITRARY,
    &border::ring_color::PLUGIN,
    &border::ring_color::PLUGIN_ARBITRARY,
    &border::ring_width::PLUGIN_INSET_1,
    &border::ring_width::PLUGIN_INSET_2,
    &border::ring_color::PLUGIN_INSET_1,
    &border::ring_color::PLUGIN_INSET_2,
    &border::ring_offset_width::PLUGIN,
    &border::ring_offset_width::PLUGIN_ARBITRARY,
    &border::ring_offset_color::PLUGIN,
    &border::ring_offset_color::PLUGIN_ARBITRARY,
    &filter::blur::PLUGIN,
    &filter::blur::PLUGIN_ARBITRARY,
    &filter::brightness::PLUGIN,
    &filter::contrast::PLUGIN,
    &filter::drop_shadow::PLUGIN,
    &filter::drop_shadow::PLUGIN_ARBITRARY,
    &filter::grayscale::PLUGIN,
    &filter::hue_rotate::PLUGIN,
    &filter::hue_rotate::PLUGIN_ARBITRARY,
    &filter::invert::PLUGIN,
    &filter::saturate::PLUGIN,
    &filter::sepia::PLUGIN,
    &filter::filter_type::PLUGIN,
    &filter::backdrop_blur::PLUGIN,
    &filter::backdrop_blur::PLUGIN_ARBITRARY,
    &filter::backdrop_brightness::PLUGIN,
    &filter::backdrop_contrast::PLUGIN,
    &filter::backdrop_grayscale::PLUGIN,
    &filter::backdrop_hue_rotate::PLUGIN,
    &filter::backdrop_hue_rotate::PLUGIN_ARBITRARY,
    &filter::backdrop_invert::PLUGIN,
    &filter::backdrop_saturate::PLUGIN,
    &filter::backdrop_sepia::PLUGIN,
    &filter::backdrop_filter::PLUGIN,
    &transition::transition_property::PLUGIN,
    &transition::transition_property::PLUGIN_ARBITRARY,
    &transition::transition_delay::PLUGIN,
    &transition::transition_delay::PLUGIN_ARBITRARY,
    &transition::transition_duration::PLUGIN,
    &transition::transition_duration::PLUGIN_ARBITRARY,
    &transition::transition_timing_function::PLUGIN,
    &transition::transition_timing_function::PLUGIN_ARBITRARY,
    &interactivity::will_change::PLUGIN,
    &interactivity::will_change::PLUGIN_ARBITRARY,
    &typography::content::PLUGIN,
    &typography::content::PLUGIN_ARBITRARY,
    &typography::line_clamp::PLUGIN_NUMBER,
    &typography::line_clamp::PLUGIN_LIST,
    &layout::at_container::PLUGIN,
    &layout::at_container::PLUGIN_ARBITRARY,
];

/// Configuration for the [`Theme::dark_mode`] field.
///
/// It defines how the `dark:` variant should behave.
///
/// The default value is [`DarkMode::Media`] which enables the automatic detection of the theme based
/// on  user preference.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum DarkMode {
    /// The `dark:` variant will modify the class of the selector. You'll then need to toggle this
    /// class to enable the dark theme.
    ///
    /// # Example
    ///
    /// ```
    /// use encre_css::{Config, config::DarkMode};
    ///
    /// let mut config = Config::default();
    /// config.theme.dark_mode = DarkMode::new_class("body.dark");
    ///
    /// let generated = encre_css::generate(
    ///     ["dark:text-white"],
    ///     &config,
    /// );
    ///
    /// assert!(generated.ends_with(r#"body.dark .dark\:text-white {
    ///   color: #fff;
    /// }"#));
    /// ```
    Class(Cow<'static, str>),

    /// The `dark:` variant will generates a `@media (prefers-color-scheme: dark)` rule to enable
    /// the dark theme following user preference.
    ///
    /// This is the default value.
    ///
    /// # Example
    ///
    /// ```
    /// use encre_css::{Config, config::DarkMode};
    ///
    /// let mut config = Config::default();
    /// config.theme.dark_mode = DarkMode::Media;
    ///
    /// let generated = encre_css::generate(
    ///     ["dark:text-white"],
    ///     &config,
    /// );
    ///
    /// assert!(generated.ends_with(r#"@media (prefers-color-scheme: dark) {
    ///   .dark\:text-white {
    ///     color: #fff;
    ///   }
    /// }"#));
    /// ```
    #[default]
    Media,
}

impl DarkMode {
    /// Quickly build a [`DarkMode::Class`] value.
    pub fn new_class<T: Into<Cow<'static, str>>>(class: T) -> Self {
        Self::Class(class.into())
    }
}

/// Configuration for the [`Theme::aria`] field.
///
/// It defines a list of custom ARIA states.
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct Aria(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

impl Aria {
    /// Add an ARIA state to the list.
    #[inline]
    pub fn add<T1: Into<Cow<'static, str>>, T2: Into<Cow<'static, str>>>(
        &mut self,
        key: T1,
        val: T2,
    ) {
        self.0.insert(key.into(), val.into());
    }

    /// Remove an ARIA state from the list.
    #[inline]
    pub fn remove<T: Into<Cow<'static, str>>>(&mut self, key: T) {
        self.0.remove(&key.into());
    }

    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&Cow<'static, str>, &Cow<'static, str>)> {
        self.0.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

/// Configuration for the [`Theme::screens`] field.
///
/// It defines a list of custom screen breakpoints.
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct Screens(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

impl Screens {
    /// Add a custom screen breakpoint to the list.
    #[inline]
    pub fn add<T1: Into<Cow<'static, str>>, T2: Into<Cow<'static, str>>>(
        &mut self,
        key: T1,
        val: T2,
    ) {
        self.0.insert(key.into(), val.into());
    }

    /// Remove a custom screen breakpoint from the list.
    #[inline]
    pub fn remove<T: Into<Cow<'static, str>>>(&mut self, key: T) {
        self.0.remove(&key.into());
    }

    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&Cow<'static, str>, &Cow<'static, str>)> {
        self.0.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

/// Configuration for the [`Theme::containers`] field.
///
/// It defines a list of custom container size breakpoints.
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct Containers(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

impl Containers {
    /// Add a custom container size breakpoint to the list.
    #[inline]
    pub fn add<T1: Into<Cow<'static, str>>, T2: Into<Cow<'static, str>>>(
        &mut self,
        key: T1,
        val: T2,
    ) {
        self.0.insert(key.into(), val.into());
    }

    /// Remove a custom container size breakpoint from the list.
    #[inline]
    pub fn remove<T: Into<Cow<'static, str>>>(&mut self, key: T) {
        self.0.remove(&key.into());
    }

    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&Cow<'static, str>, &Cow<'static, str>)> {
        self.0.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

/// Configuration for the [`Theme::colors`] field.
///
/// It defines a list of custom colors.
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct Colors(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

impl Colors {
    /// Add a custom color to the list.
    #[inline]
    pub fn add<T1: Into<Cow<'static, str>>, T2: Into<Cow<'static, str>>>(
        &mut self,
        key: T1,
        val: T2,
    ) {
        self.0.insert(key.into(), val.into());
    }

    /// Remove a custom color from the list.
    #[inline]
    pub fn remove<T: Into<Cow<'static, str>>>(&mut self, key: T) {
        self.0.remove(&key.into());
    }

    #[inline]
    pub(crate) fn get<'a, T: Into<Cow<'a, str>>>(&self, key: T) -> Option<&Cow<'a, str>> {
        self.0.get(&key.into())
    }

    #[inline]
    pub(crate) fn contains<'a, T: Into<Cow<'a, str>>>(&self, key: T) -> bool {
        self.0.contains_key(&key.into())
    }
}

/// Configuration for the [`Config::shortcuts`] field.
///
/// It defines a list of shortcuts used to combine several utility classes into one.
///
/// # Example
///
/// ```
/// use encre_css::Config;
///
/// let mut config = Config::default();
/// config.shortcuts.add("btn", "border-1 rounded-xl bg-red-500");
///
/// let generated = encre_css::generate(
///     [r#"<button class="btn">Click me</button>"#],
///     &config,
/// );
///
/// assert!(generated.ends_with(r#".btn {
///   border-radius: 0.75rem;
/// }
///
/// .btn {
///   border-width: 1px;
/// }
///
/// .btn {
///   background-color: oklch(63.7% .237 25.331);
/// }"#));
/// ```
///
/// ### Corresponding TOML configuration
///
/// <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[shortcuts]</span>
/// btn = <span class="string">"border-1 rounded-xl bg-red-500"</span></code></pre></div>
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct Shortcuts(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

impl Shortcuts {
    /// Add a shortcut to the list.
    #[inline]
    pub fn add<T1: Into<Cow<'static, str>>, T2: Into<Cow<'static, str>>>(
        &mut self,
        key: T1,
        val: T2,
    ) {
        self.0.insert(key.into(), val.into());
    }

    /// Remove a shortcut from the list.
    #[inline]
    pub fn remove<T: Into<Cow<'static, str>>>(&mut self, key: T) {
        self.0.remove(&key.into());
    }

    #[inline]
    pub(crate) fn get<'a, T: Into<Cow<'a, str>>>(&self, key: T) -> Option<&Cow<'a, str>> {
        self.0.get(&key.into())
    }
}

/// Configuration for the [`Config::layers`] field.
///
/// It defines a list of layers used to change the ordering of the generated classes.
///
/// ### Layers and ordering
///
/// By default, every class is ordered based on its plugin, variants and modifier.
///
/// If you want more control over the ordering of a specific set of classes, you can define a
/// _layer_ in the configuration (see the example below) and use the variant `l-<layer_name>` to
/// put the class in the corresponding layer.
///
/// By default, the layer containing builtin plugins has the index `0` and the layer containing
/// custom plugins has index `-1`.
///
/// The index is an `i8`, so it's between -128 and 127.
///
/// # Example
///
/// ```
/// use encre_css::Config;
///
/// let mut config = Config::default();
/// config.layers.add("components", -2);
/// config.layers.add("utilities", 2);
///
/// let generated = encre_css::generate(
///     [r#"<button class="l-components:bg-red-500 l-utilities:bg-red-100">Click me</button>"#],
///     &config,
/// );
///
/// // Without the use of layers, `bg-red-100` would have been generated first and would have been
/// // overridden by `bg-red-500`.
/// // Here the layer `components` has an index smaller than the layer
/// // `utilities`, so all the classes on this layer will be generated first.
/// assert!(generated.ends_with(r#".l-components\:bg-red-500 {
///   background-color: oklch(63.7% .237 25.331);
/// }
///
/// .l-utilities\:bg-red-100 {
///   background-color: oklch(93.6% .032 17.717);
/// }"#));
/// ```
///
/// ### Corresponding TOML configuration
///
/// <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[layers]</span>
/// components = <span class="number">-2</span>
/// utilities = <span class="number">2</span></code></pre></div>
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct Layers(BTreeMap<Cow<'static, str>, i8>);

impl Layers {
    /// Add a layer to the list.
    #[inline]
    pub fn add<T1: Into<Cow<'static, str>>>(&mut self, key: T1, val: i8) {
        self.0.insert(key.into(), val);
    }

    /// Remove a layer from the list.
    #[inline]
    pub fn remove<T: Into<Cow<'static, str>>>(&mut self, key: T) {
        self.0.remove(&key.into());
    }

    #[inline]
    pub(crate) fn get<'a, T: Into<Cow<'a, str>>>(&'a self, key: T) -> Option<&'a i8> {
        self.0.get(&key.into())
    }
}

/// The maximum depth at which shortcuts will be resolved.
///
/// During the shortcut expansion, if the depth is greater than `max_shortcut_depth`, only the already expanded selectors until the maximum depth will be added.
///
/// By default it is set to `5`.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub struct MaxShortcutDepth(usize);

impl MaxShortcutDepth {
    /// Create a new `MaxShortcutDepth`.
    pub fn new(v: usize) -> Self {
        Self(v)
    }

    /// Get the inner depth as an `usize`.
    pub fn get(&self) -> usize {
        self.0
    }
}

impl From<usize> for MaxShortcutDepth {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

impl From<MaxShortcutDepth> for usize {
    fn from(val: MaxShortcutDepth) -> Self {
        val.0
    }
}

impl Default for MaxShortcutDepth {
    fn default() -> Self {
        Self(5)
    }
}

/// Configuration for the [`Config::safelist`] field.
///
/// It defines a list of selectors that are manually forced to be present in the generated CSS.
/// It should be used when you dynamically create selectors (for example, in Javascript
/// `text-${ active ? "blue" : "gray" }-400`, in this case, `text-blue-400` and `text-gray-400`
/// should be added to the safelist).
///
/// # Example
///
/// ```
/// use encre_css::Config;
///
/// let mut config = Config::default();
/// config.safelist.add("text-blue-400");
/// config.safelist.add("text-gray-400");
///
/// let generated = encre_css::generate(
///     [r#"<button class="bg-red-500">Click me</button>"#],
///     &config,
/// );
///
/// assert!(generated.ends_with(r#".bg-red-500 {
///   background-color: oklch(63.7% .237 25.331);
/// }
///
/// .text-blue-400 {
///   color: oklch(70.7% .165 254.624);
/// }
///
/// .text-gray-400 {
///   color: oklch(70.7% .022 261.325);
/// }"#));
/// ```
///
/// ### Corresponding TOML configuration
///
/// <div class="example-wrap"><pre class="rust rust-example-rendered"><code>safelist = [<span class="string">"text-blue-400"</span>, <span class="string">"text-gray-400"</span>]</code></pre></div>
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct Safelist(BTreeSet<Cow<'static, str>>);

impl Safelist {
    /// Add a selector to the safelist.
    #[inline]
    pub fn add<T: Into<Cow<'static, str>>>(&mut self, val: T) {
        self.0.insert(val.into());
    }

    /// Remove a selector from the safelist.
    #[inline]
    pub fn remove<T: Into<Cow<'static, str>>>(&mut self, val: T) {
        self.0.remove(&val.into());
    }

    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Cow<'static, str>> {
        self.0.iter()
    }
}

/// Configuration for the [`Config::extra`] field.
///
/// It defines some extra fields that can be used to store arbitrary values usable in plugins.
/// The fields are represented as [`toml::Value`] to allow all types to be serialized.
/// It is recommended to use a table by plugin (e.g. the `encre-css-icons`'s plugin uses the
/// `icons` key containing a table grouping all configuration fields).
#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct Extra(BTreeMap<Cow<'static, str>, toml::Value>);

impl Extra {
    /// Add an extra field.
    #[inline]
    pub fn add<T1: Into<Cow<'static, str>>, T2: Into<toml::Value>>(&mut self, key: T1, val: T2) {
        self.0.insert(key.into(), val.into());
    }

    /// Remove an extra field.
    #[inline]
    pub fn remove<T: Into<Cow<'static, str>>>(&mut self, key: T) {
        self.0.remove(&key.into());
    }

    /// Get the value of an extra field.
    #[inline]
    pub fn get<'a, T: Into<Cow<'a, str>>>(&'a self, key: T) -> Option<&'a toml::Value> {
        self.0.get(&key.into())
    }
}

/// Configuration for the [`Config::theme`] field.
///
/// It defines some design system specific values like custom colors or screen breakpoints.
///
/// # Example
///
/// ```
/// use encre_css::{Config, config::DarkMode};
///
/// let mut config = Config::default();
/// config.theme.dark_mode = DarkMode::new_class("body.dark");
/// config.theme.colors.add("primary", "#d3198c");
/// config.theme.screens.add("tablet", "640px");
/// config.theme.containers.add("medium", "640px");
/// config.theme.aria.add("current", r#"current="page""#);
///
/// let generated = encre_css::generate(
///     [r#"<div class="bg-primary tablet:block aria-current:text-primary"><button class="bg-white @medium:flex">Click me</button>"#],
///     &config,
/// );
///
/// assert!(generated.ends_with(r#"
/// .bg-primary {
///   background-color: #d3198c;
/// }
///
/// .bg-white {
///   background-color: #fff;
/// }
///
/// @media (width >= 640px) {
///   .tablet\:block {
///     display: block;
///   }
/// }
///
/// @container (width >= 640px) {
///   .\@medium\:flex {
///     display: flex;
///   }
/// }
///
/// .aria-current\:text-primary[aria-current="page"] {
///   color: #d3198c;
/// }"#));
/// ```
///
/// ### Corresponding TOML configuration
///
/// <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">[theme]</span>
/// dark_mode = { class = <span class="string">"body.dark"</span> }<br>
/// <span class="kw">[theme.colors]</span>
/// primary = <span class="string">"#d3198c"</span><br>
/// <span class="kw">[theme.screens]</span>
/// tablet = <span class="string">"640px"</span><br>
/// <span class="kw">[theme.containers]</span>
/// medium = <span class="string">"640px"</span><br>
/// <span class="kw">[theme.aria]</span>
/// current = <span class="string">'current="page"'</span></code></pre></div>
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct Theme {
    /// Dark mode configuration.
    ///
    /// The default value is [`DarkMode::Media`].
    #[serde(default)]
    pub dark_mode: DarkMode,

    /// Custom screen breakpoints configuration.
    ///
    /// The default value is an empty map.
    #[serde(default)]
    pub screens: Screens,

    /// Custom container size breakpoints configuration.
    ///
    /// The default value is an empty map.
    #[serde(default)]
    pub containers: Containers,

    /// Custom colors configuration.
    ///
    /// The default value is an empty map.
    #[serde(default)]
    pub colors: Colors,

    /// Custom ARIA states.
    ///
    /// The default value is an empty map.
    #[serde(default)]
    pub aria: Aria,
}

/// The configuration of the CSS generation done in the [`generate`] function.
///
/// You can create a configuration using one of the ways listed below:
///
/// - It can be the default one:
///
/// ```
/// use encre_css::Config;
///
/// let config = Config::default();
/// let _generated = encre_css::generate([], &config);
/// ```
///
/// - It can be a customized one:
///
/// ```
/// use encre_css::Config;
///
/// let mut config = Config::default();
/// config.theme.colors.add("flashy", "#ff2d20");
///
/// let _generated = encre_css::generate([], &config);
/// ```
///
/// - It can be loaded from a [TOML](https://toml.io) file:
///
/// <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="comment"># encre-css.toml</span>
/// <span class="kw">[theme]</span>
/// dark_mode = { class = <span class="string">".dark"</span> }
/// screens = { 3xl = <span class="string">"1600px"</span>, lg = <span class="string">"2000px"</span> }<br>
/// <span class="kw">[theme.colors]</span>
/// primary = <span class="string">"#e5186a"</span>
/// yellow-400 = <span class="string">"#ffef0e"</span></code></pre></div>
///
/// ```no_run
/// use encre_css::Config;
///
/// # fn main() -> encre_css::Result<()> {
/// let config = Config::from_file("encre-css.toml")?;
/// let _generated = encre_css::generate([], &config);
/// # Ok(())
/// # }
/// ```
///
/// Based on [Tailwind v3's configuration](https://v3.tailwindcss.com/docs/configuration).
///
/// [`generate`]: crate::generate
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Safelist configuration.
    #[serde(default)]
    pub safelist: Safelist,

    /// Theme configuration.
    #[serde(default)]
    pub theme: Theme,

    /// Preflight configuration.
    #[serde(default)]
    pub preflight: Preflight,

    /// Shortcuts configuration.
    #[serde(default)]
    pub shortcuts: Shortcuts,

    /// Layers configuration.
    #[serde(default)]
    pub layers: Layers,

    /// The maximum depth at which shortcuts will be resolved.
    #[serde(default)]
    pub max_shortcut_depth: MaxShortcutDepth,

    /// Extra fields configuration.
    #[serde(default)]
    pub extra: Extra,

    /// A list of custom plugins.
    #[serde(default)]
    pub(crate) custom_plugins: Vec<CustomPlugin>,

    /// A custom scanner used to scan content.
    ///
    /// This field is skipped when deserializing from a [TOML](https://toml.io) file.
    #[serde(skip)]
    pub scanner: Scanner,

    /// A list of custom variants.
    ///
    /// Check [`BUILTIN_VARIANTS`] to choose the order of the custom variants you define.
    ///
    /// This field is skipped when deserializing from a [TOML](https://toml.io) file.
    #[serde(skip)]
    pub(crate) custom_variants: Vec<(Cow<'static, str>, Variant<'static>)>,
    // TODO: Prefix (en-)
}

impl Config {
    /// Get variants derived from other configuration fields like breakpoints and the dark mode.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn get_derived_variants(&self) -> Vec<(Cow<'static, str>, Variant<'static>)> {
        self.theme
            .screens
            .iter()
            .map(|screen| {
                (
                    screen.0.clone(),
                    Variant {
                        order: BUILTIN_VARIANTS.len(),
                        prefixed: false,
                        template: Cow::Owned(format!("@media (width >= {})", screen.1)),
                    },
                )
            })
            .chain(BUILTIN_SCREENS.iter().map(|screen| {
                (
                    Cow::from(screen.0),
                    Variant {
                        order: BUILTIN_VARIANTS.len() + self.theme.screens.0.len(),
                        prefixed: false,
                        template: Cow::Owned(format!("@media (width >= {})", screen.1)),
                    },
                )
            }))
            .chain(self.theme.screens.iter().map(|screen| {
                (
                    Cow::from(format!("max-{}", screen.0)),
                    Variant {
                        order: BUILTIN_VARIANTS.len()
                            + self.theme.screens.len()
                            + BUILTIN_SCREENS.len(),
                        prefixed: false,
                        template: Cow::Owned(format!("@media (width < {})", screen.1)),
                    },
                )
            }))
            .chain(BUILTIN_SCREENS.iter().map(|screen| {
                (
                    Cow::from(format!("max-{}", screen.0)),
                    Variant {
                        order: BUILTIN_VARIANTS.len()
                            + 2 * self.theme.screens.len()
                            + BUILTIN_SCREENS.len(),
                        prefixed: false,
                        template: Cow::Owned(format!("@media (width < {})", screen.1)),
                    },
                )
            }))
            .chain(self.theme.containers.iter().map(|container| {
                (
                    Cow::from(format!("@{}", container.0)),
                    Variant {
                        order: BUILTIN_VARIANTS.len()
                            + 2 * self.theme.screens.len()
                            + 2 * BUILTIN_SCREENS.len(),
                        prefixed: false,
                        template: Cow::Owned(format!("@container (width >= {})", container.1)),
                    },
                )
            }))
            .chain(BUILTIN_CONTAINERS.iter().map(|container| {
                (
                    Cow::from(format!("@{}", container.0)),
                    Variant {
                        order: BUILTIN_VARIANTS.len()
                            + 2 * self.theme.screens.len()
                            + 2 * BUILTIN_SCREENS.len()
                            + self.theme.containers.len(),
                        prefixed: false,
                        template: Cow::Owned(format!("@container (width >= {})", container.1)),
                    },
                )
            }))
            .chain(self.theme.containers.iter().map(|container| {
                (
                    Cow::from(format!("@max-{}", container.0)),
                    Variant {
                        order: BUILTIN_VARIANTS.len()
                            + 2 * self.theme.screens.len()
                            + 2 * BUILTIN_SCREENS.len()
                            + self.theme.containers.len()
                            + BUILTIN_CONTAINERS.len(),
                        prefixed: false,
                        template: Cow::Owned(format!("@container (width < {})", container.1)),
                    },
                )
            }))
            .chain(BUILTIN_CONTAINERS.iter().map(|container| {
                (
                    Cow::from(format!("@max-{}", container.0)),
                    Variant {
                        order: BUILTIN_VARIANTS.len()
                            + 2 * self.theme.screens.len()
                            + 2 * BUILTIN_SCREENS.len()
                            + 2 * self.theme.containers.len()
                            + BUILTIN_CONTAINERS.len(),
                        prefixed: false,
                        template: Cow::Owned(format!("@container (width < {})", container.1)),
                    },
                )
            }))
            .chain(self.theme.aria.iter().map(|aria| {
                (
                    Cow::from(format!("aria-{}", aria.0)),
                    Variant {
                        order: BUILTIN_VARIANTS.len()
                            + 2 * self.theme.screens.len()
                            + 2 * BUILTIN_SCREENS.len()
                            + 2 * self.theme.containers.len()
                            + 2 * BUILTIN_CONTAINERS.len(),
                        prefixed: false,
                        template: Cow::Owned(format!("&[aria-{}]", aria.1)),
                    },
                )
            }))
            .chain(iter::once(match &self.theme.dark_mode {
                DarkMode::Media => (
                    Cow::from("dark"),
                    Variant {
                        order: BUILTIN_VARIANTS.len()
                            + 2 * self.theme.screens.len()
                            + 2 * BUILTIN_SCREENS.len()
                            + 2 * self.theme.containers.len()
                            + 2 * BUILTIN_CONTAINERS.len()
                            + self.theme.aria.0.len(),
                        prefixed: false,
                        template: Cow::Borrowed("@media (prefers-color-scheme: dark)"),
                    },
                ),
                DarkMode::Class(name) => (
                    Cow::from("dark"),
                    Variant {
                        order: BUILTIN_VARIANTS.len()
                            + 2 * self.theme.screens.len()
                            + 2 * BUILTIN_SCREENS.len()
                            + 2 * self.theme.containers.len()
                            + 2 * BUILTIN_CONTAINERS.len()
                            + self.theme.aria.len(),
                        prefixed: false,
                        template: Cow::Owned(format!("{name} &")),
                    },
                ),
            }))
            .collect()
    }

    /// Returns the order of the last variant which can be used when defining a new variant which
    /// must be generated after all the other variants.
    ///
    /// Note that if you are not the maintainer of a crate providing variants, you can ignore this
    /// function.
    pub fn last_variant_order(&self) -> usize {
        BUILTIN_VARIANTS.len()
            + 2 * self.theme.screens.len()
            + 2 * BUILTIN_SCREENS.len()
            + 2 * self.theme.containers.len()
            + 2 * BUILTIN_CONTAINERS.len()
            + self.theme.aria.len()
            + 1
            + self.custom_variants.len()
    }

    /// Register a custom plugin which will be used during CSS generation.
    ///
    /// Note that if you are not the maintainer of a crate providing plugins, you can ignore this
    /// function, see [`crate::plugins`].
    ///
    /// See [`crate::plugins::Plugin`] to learn how to write plugins and
    /// [`crate::plugins::PluginKind`] for choosing the correct plugin kind for your use case.
    ///
    /// # Example
    ///
    /// ```
    /// use encre_css::{Config, prelude::build_plugin::*};
    ///
    /// const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    ///     prop: SingleProp("color"),
    ///     values: map! {
    ///         "prose" => "#333",
    ///         "prose-invert" => "#eee",
    ///     },
    /// });
    ///
    /// let mut config = Config::default();
    /// config.register_plugin(&PLUGIN);
    ///
    /// let generated = encre_css::generate(
    ///     ["prose", "prose-invert"],
    ///     &config,
    /// );
    ///
    /// assert!(generated.ends_with(".prose {
    ///   color: #333;
    /// }
    ///
    /// .prose-invert {
    ///   color: #eee;
    /// }"));
    /// ```
    pub fn register_plugin(&mut self, plugin: &'static StaticPlugin) {
        self.custom_plugins.push(CustomPlugin::Static(plugin));
    }

    pub fn register_dynamic_plugin(&mut self, plugin: DynamicPlugin) {
        self.custom_plugins.push(CustomPlugin::Dynamic(plugin));
    }

    /// Register a custom variant which will be used during CSS generation.
    ///
    /// Note that if you are not the maintainer of a crate providing variants, you can ignore this
    /// function.
    ///
    /// # Example
    ///
    /// ```
    /// use encre_css::{Config, selector::Variant};
    /// use std::borrow::Cow;
    ///
    /// let mut config = Config::default();
    /// config.register_variant(
    ///     "headings",
    ///     // Insert the classes having this variant after all the other variants
    ///     Variant::new(config.last_variant_order(), "& :where(h1, h2, h3, h4, h5, h6)")
    /// );
    ///
    /// let generated = encre_css::generate(
    ///     ["headings:text-gray-700"],
    ///     &config,
    /// );
    ///
    /// assert!(generated.ends_with(".headings\\:text-gray-700 :where(h1, h2, h3, h4, h5, h6) {
    ///   color: oklch(37.3% .034 259.733);
    /// }"));
    /// ```
    ///
    /// You can also make a prefixed variant, that is a variant which has a prefix and an arbitrary
    /// value delimited by square brackets. When defining this kind of variant, you need to call
    /// [`Variant::with_prefixed`] and to insert the placeholder `{}` in the variant
    /// template, it will be replaced with the given arbitrary value.
    ///
    /// # Example
    ///
    /// ```
    /// use encre_css::{Config, selector::Variant};
    /// use std::borrow::Cow;
    ///
    /// let mut config = Config::default();
    /// config.register_variant(
    ///     "media",
    ///     // Insert the classes having this variant after all the other variants
    ///     Variant::new(config.last_variant_order(), "@media {}").with_prefixed()
    /// );
    ///
    /// let generated = encre_css::generate(
    ///     ["media-[print]:flex"],
    ///     &config,
    /// );
    ///
    /// assert!(generated.ends_with(r"@media print {
    ///   .media-\[print\]\:flex {
    ///     display: flex;
    ///   }
    /// }"));
    /// ```
    pub fn register_variant<T: Into<Cow<'static, str>>>(
        &mut self,
        variant_name: T,
        variant: Variant<'static>,
    ) {
        self.custom_variants.push((variant_name.into(), variant));
    }

    /// Deserialize the content of a [TOML](https://toml.io) file to get the configuration.
    ///
    /// # Example
    ///
    /// <div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="comment"># encre-css.toml</span>
    /// <span class="kw">[theme]</span>
    /// dark_mode = { type = <span class="string">"class"</span>, class = <span class="string">".dark"</span> }
    /// screens = { 3xl = <span class="string">"1600px"</span>, lg = <span class="string">"2000px"</span> }<br>
    /// <span class="kw">[theme.colors]</span>
    /// primary = <span class="string">"#e5186a"</span>
    /// yellow-400 = <span class="string">"#ffef0e"</span></code></pre></div>
    ///
    /// ```no_run
    /// use encre_css::Config;
    ///
    /// # fn main() -> encre_css::Result<()> {
    /// let config = Config::from_file("encre-css.toml")?;
    /// let _generated = encre_css::generate([], &config);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See [`Config`] for other ways of creating a configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ConfigFileNotFound`] if the given file does not exist.
    /// Returns [`Error::ConfigParsing`] if the given file could not be parsed.
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        Ok(toml::from_str(&fs::read_to_string(&path).map_err(
            |e| Error::ConfigFileNotFound(path.as_ref().to_path_buf(), e),
        )?)?)
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.safelist == other.safelist
            && self.theme == other.theme
            && self.preflight == other.preflight
            && self.shortcuts == other.shortcuts
            && self.max_shortcut_depth == other.max_shortcut_depth
            && self.extra == other.extra
            && self.custom_variants == other.custom_variants
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("safelist", &self.safelist)
            .field("theme", &self.theme)
            .field("preflight", &self.preflight)
            .field("shortcuts", &self.shortcuts)
            .field("max_shortcut_depth", &self.max_shortcut_depth)
            .field("extra", &self.extra)
            .field("custom_plugins", &self.custom_plugins)
            .field("custom_variants", &self.custom_variants)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn gen_css_with_custom_config() {
        let mut config = base_config();
        config.theme.colors.add("rosa-500", "#e5186a");
        config.theme.screens.add("3xl", "1600px");

        let generated = generate(["3xl:text-rosa-500"], &config);

        assert_eq!(
            generated,
            String::from(
                r"@media (width >= 1600px) {
  .\33xl\:text-rosa-500 {
    color: #e5186a;
  }
}"
            )
        );
    }

    #[test]
    fn gen_css_with_shortcuts() {
        let mut config = base_config();
        config
            .shortcuts
            .add("btn", "bg-red-500 border-1 rounded-xl");
        config.shortcuts.add("bg", "bg-blue-100");

        let generated = generate(["btn", "bg-yellow-500"], &config);

        assert_eq!(
            generated,
            String::from(
                ".btn {
  border-radius: 0.75rem;
}

.btn {
  border-width: 1px;
}

.bg-yellow-500 {
  background-color: oklch(79.5% .184 86.047);
}

.btn {
  background-color: oklch(63.7% .237 25.331);
}"
            )
        );
    }

    #[test]
    fn gen_css_with_nested_shortcuts() {
        let mut config = base_config();
        config
            .shortcuts
            .add("btn", "bg-red-500 border-1 rounded-xl");
        config.shortcuts.add("btn-primary", "btn bg-blue-500");

        let generated = generate(["btn-primary"], &config);

        assert_eq!(
            generated,
            String::from(
                ".btn-primary {
  border-radius: 0.75rem;
}

.btn-primary {
  border-width: 1px;
}

.btn-primary {
  background-color: oklch(62.3% .214 259.815);
}

.btn-primary {
  background-color: oklch(63.7% .237 25.331);
}"
            )
        );
    }

    #[test]
    fn gen_css_with_shortcut_cycle() {
        let mut config = base_config();
        config
            .shortcuts
            .add("btn", "bg-red-500 border-1 rounded-xl btn-primary");
        config.shortcuts.add("btn-primary", "btn bg-blue-500");

        let generated = generate(["btn-primary"], &config);

        assert_eq!(
            generated,
            String::from(
                ".btn-primary {
  border-radius: 0.75rem;
}

.btn-primary {
  border-width: 1px;
}

.btn-primary {
  background-color: oklch(62.3% .214 259.815);
}

.btn-primary {
  background-color: oklch(63.7% .237 25.331);
}"
            )
        );
    }

    #[test]
    fn gen_css_with_safelist() {
        let mut config = base_config();
        config.safelist.add("text-red-500");
        config.safelist.add("btn");
        config.shortcuts.add("btn", "text-red-400");

        let generated = generate(["bg-red-300"], &config);

        assert_eq!(
            generated,
            String::from(
                ".bg-red-300 {
  background-color: oklch(80.8% .114 19.571);
}

.btn {
  color: oklch(70.4% .191 22.216);
}

.text-red-500 {
  color: oklch(63.7% .237 25.331);
}"
            )
        );
    }

    #[test]
    fn gen_css_with_custom_plugin() {
        use crate::prelude::build_plugin::*;

        const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
            prop: SingleProp("content"),
            values: map! {
                "emoji-tada" => "\"\u{1f389}\"",
                "emoji-rocket" => "\"\u{1f680}\"",
            },
        });

        let mut config = base_config();
        config.register_plugin(&PLUGIN);

        let generated = generate(["emoji-tada"], &config);

        assert_eq!(
            generated,
            String::from(
                ".emoji-tada {
  content: \"\u{1f389}\";
}"
            )
        );
    }

    #[test]
    fn gen_css_with_dynamic_plugin() {
        use crate::prelude::build_plugin::*;

        let mut config = Config::default();
        config.preflight = Preflight::None;
        config.register_dynamic_plugin(Plugin::new_dynamic(PluginKind::ListValues {
            prop: DynamicPropertyName::SingleProp(String::from("content")),
            values: HashMap::from([
                (String::from("emoji-tada"), String::from("\"\u{1f389}\"")),
                (String::from("emoji-rocket"), String::from("\"\u{1f680}\"")),
            ]),
        }));

        let generated = generate(["emoji-tada"], &config);

        assert_eq!(
            generated,
            String::from(
                ".emoji-tada {
  content: \"\u{1f389}\";
}"
            )
        );
    }

    #[test]
    fn gen_css_with_custom_parsed_plugin() {
        let config = match Config::from_file("tests/fixtures/custom-plugin-config.toml") {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        };

        let generated = generate(["emoji-tada"], &config);

        assert_eq!(
            generated,
            String::from(
                ".emoji-tada {
  content: \"\u{1f389}\";
}"
            )
        );
    }

    #[test]
    fn gen_css_with_custom_parsed_plugin_and_multiple_props() {
        let config = match Config::from_file("tests/fixtures/custom-plugin-config-multiple.toml") {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        };

        let generated = generate(["shape-roof-1"], &config);

        assert_eq!(
            generated,
            String::from(
                ".shape-roof-1 {
  border-inline: 1px;
  border-top: 1px;
}"
            )
        );
    }

    #[test]
    fn config_is_extended_and_overridden() {
        let config = Config::from_file("tests/fixtures/custom-config.toml").unwrap();

        let generated = generate(
            [
                "bg-rosa-500",
                "bg-yellow-400",
                "bg-yellow-100",
                "3xl:underline",
                "lg:text-rosa-500",
            ],
            &config,
        );

        assert_eq!(
            generated,
            String::from(
                r".bg-rosa-500 {
  background-color: #e5186a;
}

.bg-yellow-100 {
  background-color: oklch(97.3% .071 103.193);
}

.bg-yellow-400 {
  background-color: #ffef0e;
}

@media (width >= 2000px) {
  .lg\:text-rosa-500 {
    color: #e5186a;
  }
}

@media (width >= 1600px) {
  .\33xl\:underline {
    -webkit-text-decoration-line: underline;
    text-decoration-line: underline;
  }
}"
            )
        );
    }

    #[test]
    fn deserialize_config() {
        let mut config = base_config();
        config.theme.colors.add("rosa-500", "#e5186a");
        config.theme.colors.add("yellow-400", "#ffef0e");
        config.theme.aria.add("current", "current=\"page\"");
        config.theme.screens.add("lg", "2000px");
        config.theme.screens.add("3xl", "1600px");
        config.theme.dark_mode = DarkMode::new_class(".dark");

        assert_eq!(
            Config::from_file("tests/fixtures/custom-config.toml").unwrap(),
            config
        );
    }

    #[test]
    fn serialize_config() {
        let mut config = Config {
            preflight: Preflight::None,
            ..Default::default()
        };
        config.theme.dark_mode = DarkMode::new_class(".dark");
        config.theme.screens.add("3xl", "1600px");
        config.theme.screens.add("lg", "2000px");
        config.theme.colors.add("rosa-500", "#e5186a");
        config.theme.colors.add("yellow-400", "#ffef0e");
        config.theme.aria.add("current", "current=\"page\"");

        let result = toml::to_string(&config).unwrap();

        let expected_config = fs::read_to_string("tests/fixtures/custom-config.toml").unwrap();
        assert_eq!(expected_config, result);
    }

    #[test]
    fn toml_doc_tests() {
        // This function tests all TOML blocks used in the documentation
        // If a block is changed in this function, it should also be changed in the corresponding
        // documentation section and vice-versa

        // Shortcuts
        {
            let toml_for_shortcuts = r#"
        [shortcuts]
        btn = "border-1 rounded-xl bg-red-500"
        "#;
            let config: Config = toml::from_str(toml_for_shortcuts).unwrap();

            let generated = generate([r#"<button class="btn">Click me</button>"#], &config);

            assert!(generated.ends_with(
                r#".btn {
  border-radius: 0.75rem;
}

.btn {
  border-width: 1px;
}

.btn {
  background-color: oklch(63.7% .237 25.331);
}"#
            ));
        }

        // Safelist
        {
            let toml_for_safelist = r#"safelist = ["text-blue-400", "text-gray-400"]"#;
            let config: Config = toml::from_str(toml_for_safelist).unwrap();

            let generated = generate([r#"<button class="bg-red-500">Click me</button>"#], &config);

            assert!(generated.ends_with(
                r#".bg-red-500 {
  background-color: oklch(63.7% .237 25.331);
}

.text-blue-400 {
  color: oklch(70.7% .165 254.624);
}

.text-gray-400 {
  color: oklch(70.7% .022 261.325);
}"#
            ));
        }

        // Preflight
        {
            let toml_for_preflight = r#"preflight = "none""#;
            let config: Config = toml::from_str(toml_for_preflight).unwrap();
            assert!(generate([], &config).is_empty());

            let toml_for_preflight = r#"preflight = { custom = "html, body { width: 100vw; height: 100vh; margin: 0; }" }"#;
            let config: Config = toml::from_str(toml_for_preflight).unwrap();
            assert_eq!(
                generate([], &config),
                "html, body { width: 100vw; height: 100vh; margin: 0; }"
            );

            let toml_for_preflight =
                r#"preflight = { full = { font_family_mono = "'Fira Code'" } }"#;
            let config: Config = toml::from_str(toml_for_preflight).unwrap();
            assert!(generate([], &config).contains(
                "code, kbd, samp, pre {
  font-family: 'Fira Code';"
            ));
        }

        // Theme
        {
            let toml_for_theme = r##"
            [theme]
            dark_mode = { class = "body.dark" }

            [theme.colors]
            primary = "#d3198c"

            [theme.screens]
            tablet = "640px"

            [theme.containers]
            medium = "640px"

            [theme.aria]
            current = 'current="page"'
            "##;
            let config: Config = toml::from_str(toml_for_theme).unwrap();

            let generated = generate(
                [
                    r#"<div class="bg-primary tablet:block aria-current:text-primary"><button class="bg-white @medium:flex">Click me</button>"#,
                ],
                &config,
            );

            assert!(generated.ends_with(
                r#"
.bg-primary {
  background-color: #d3198c;
}

.bg-white {
  background-color: #fff;
}

@media (width >= 640px) {
  .tablet\:block {
    display: block;
  }
}

@container (width >= 640px) {
  .\@medium\:flex {
    display: flex;
  }
}

.aria-current\:text-primary[aria-current="page"] {
  color: #d3198c;
}"#
            ));
        }
    }
}
