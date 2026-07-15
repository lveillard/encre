//! Define beautiful typographic defaults for HTML you don't control.
//!
/// CSS Styles are derived from TailwindCSS' typography plugin (https://github.com/tailwindlabs/tailwindcss-typography/blob/master/src/styles.js),
/// under the MIT license.
use encre_css::{prelude::build_plugin::*, selector::Variant, Config};
use std::borrow::Cow;

const PROSE_DEFAULT_CSS: &str = r#".prose {
  color: var(--en-prose-body);
  max-width: 65ch;
}

.prose :where([class~="lead"]):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-lead);
  font-size: 1.25em;
  line-height: 1.6;
  margin-top: 1.2em;
  margin-bottom: 1.2em;
}

.prose :where(a):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-links);
  text-decoration: underline;
  font-weight: 500;
}

.prose :where(strong):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-bold);
  font-weight: 600;
}

.prose :where(a strong):not(:where([class~="not-prose"] *)) {
  color: inherit;
}

.prose :where(blockquote strong):not(:where([class~="not-prose"] *)) {
  color: inherit;
}

.prose :where(thead th strong):not(:where([class~="not-prose"] *)) {
  color: inherit;
}

.prose :where(ol):not(:where([class~="not-prose"] *)) {
  list-style-type: decimal;
  margin-top: 1.25em;
  margin-bottom: 1.25em;
  padding-left: 1.625em;
}

.prose :where(ol[type="A"]):not(:where([class~="not-prose"] *)) {
  list-style-type: upper-alpha;
}

.prose :where(ol[type="a"]):not(:where([class~="not-prose"] *)) {
  list-style-type: lower-alpha;
}

.prose :where(ol[type="A" s]):not(:where([class~="not-prose"] *)) {
  list-style-type: upper-alpha;
}

.prose :where(ol[type="a" s]):not(:where([class~="not-prose"] *)) {
  list-style-type: lower-alpha;
}

.prose :where(ol[type="I"]):not(:where([class~="not-prose"] *)) {
  list-style-type: upper-roman;
}

.prose :where(ol[type="i"]):not(:where([class~="not-prose"] *)) {
  list-style-type: lower-roman;
}

.prose :where(ol[type="I" s]):not(:where([class~="not-prose"] *)) {
  list-style-type: upper-roman;
}

.prose :where(ol[type="i" s]):not(:where([class~="not-prose"] *)) {
  list-style-type: lower-roman;
}

.prose :where(ol[type="1"]):not(:where([class~="not-prose"] *)) {
  list-style-type: decimal;
}

.prose :where(ul):not(:where([class~="not-prose"] *)) {
  list-style-type: disc;
  margin-top: 1.25em;
  margin-bottom: 1.25em;
  padding-left: 1.625em;
}

.prose :where(ol > li):not(:where([class~="not-prose"] *))::marker {
  font-weight: 400;
  color: var(--en-prose-counters);
}

.prose :where(ul > li):not(:where([class~="not-prose"] *))::marker {
  color: var(--en-prose-bullets);
}

.prose :where(hr):not(:where([class~="not-prose"] *)) {
  border-color: var(--en-prose-hr);
  border-top-width: 1px;
  margin-top: 3em;
  margin-bottom: 3em;
}

.prose :where(blockquote):not(:where([class~="not-prose"] *)) {
  font-weight: 500;
  font-style: italic;
  color: var(--en-prose-quotes);
  border-left-width: 0.25rem;
  border-left-color: var(--en-prose-quote-borders);
  quotes: "\201C""\201D""\2018""\2019";
  margin-top: 1.6em;
  margin-bottom: 1.6em;
  padding-left: 1em;
}

.prose :where(blockquote p:first-of-type):not(:where([class~="not-prose"] *))::before {
  content: open-quote;
}

.prose :where(blockquote p:last-of-type):not(:where([class~="not-prose"] *))::after {
  content: close-quote;
}

.prose :where(h1):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-headings);
  font-weight: 800;
  font-size: 2.25em;
  margin-top: 0;
  margin-bottom: 0.8888889em;
  line-height: 1.1111111;
}

.prose :where(h1 strong):not(:where([class~="not-prose"] *)) {
  font-weight: 900;
  color: inherit;
}

.prose :where(h2):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-headings);
  font-weight: 700;
  font-size: 1.5em;
  margin-top: 2em;
  margin-bottom: 1em;
  line-height: 1.3333333;
}

.prose :where(h2 strong):not(:where([class~="not-prose"] *)) {
  font-weight: 800;
  color: inherit;
}

.prose :where(h3):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-headings);
  font-weight: 600;
  font-size: 1.25em;
  margin-top: 1.6em;
  margin-bottom: 0.6em;
  line-height: 1.6;
}

.prose :where(h3 strong):not(:where([class~="not-prose"] *)) {
  font-weight: 700;
  color: inherit;
}

.prose :where(h4):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-headings);
  font-weight: 600;
  margin-top: 1.5em;
  margin-bottom: 0.5em;
  line-height: 1.5;
}

.prose :where(h4 strong):not(:where([class~="not-prose"] *)) {
  font-weight: 700;
  color: inherit;
}

.prose :where(img):not(:where([class~="not-prose"] *)) {
  margin-top: 2em;
  margin-bottom: 2em;
}

.prose :where(figure > *):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
  margin-bottom: 0;
}

.prose :where(figcaption):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-captions);
  font-size: 0.875em;
  line-height: 1.4285714;
  margin-top: 0.8571429em;
}

.prose :where(code):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-code);
  font-weight: 600;
  font-size: 0.875em;
}

.prose :where(code):not(:where([class~="not-prose"] *))::before {
  content: "`";
}

.prose :where(code):not(:where([class~="not-prose"] *))::after {
  content: "`";
}

.prose :where(a code):not(:where([class~="not-prose"] *)) {
  color: inherit;
}

.prose :where(h1 code):not(:where([class~="not-prose"] *)) {
  color: inherit;
}

.prose :where(h2 code):not(:where([class~="not-prose"] *)) {
  color: inherit;
  font-size: 0.875em;
}

.prose :where(h3 code):not(:where([class~="not-prose"] *)) {
  color: inherit;
  font-size: 0.9em;
}

.prose :where(h4 code):not(:where([class~="not-prose"] *)) {
  color: inherit;
}

.prose :where(blockquote code):not(:where([class~="not-prose"] *)) {
  color: inherit;
}

.prose :where(thead th code):not(:where([class~="not-prose"] *)) {
  color: inherit;
}

.prose :where(pre):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-pre-code);
  background-color: var(--en-prose-pre-bg);
  overflow-x: auto;
  font-weight: 400;
  font-size: 0.875em;
  line-height: 1.7142857;
  margin-top: 1.7142857em;
  margin-bottom: 1.7142857em;
  border-radius: 0.375rem;
  padding-top: 0.8571429em;
  padding-right: 1.1428571em;
  padding-bottom: 0.8571429em;
  padding-left: 1.1428571em;
}

.prose :where(pre code):not(:where([class~="not-prose"] *)) {
  background-color: transparent;
  border-width: 0;
  border-radius: 0;
  padding: 0;
  font-weight: inherit;
  color: inherit;
  font-size: inherit;
  font-family: inherit;
  line-height: inherit;
}

.prose :where(pre code):not(:where([class~="not-prose"] *))::before {
  content: none;
}

.prose :where(pre code):not(:where([class~="not-prose"] *))::after {
  content: none;
}

.prose :where(table):not(:where([class~="not-prose"] *)) {
  width: 100%;
  table-layout: auto;
  text-align: left;
  margin-top: 2em;
  margin-bottom: 2em;
  font-size: 0.875em;
  line-height: 1.7142857;
}

.prose :where(thead):not(:where([class~="not-prose"] *)) {
  border-bottom-width: 1px;
  border-bottom-color: var(--en-prose-th-borders);
}

.prose :where(thead th):not(:where([class~="not-prose"] *)) {
  color: var(--en-prose-headings);
  font-weight: 600;
  vertical-align: bottom;
  padding-right: 0.5714286em;
  padding-bottom: 0.5714286em;
  padding-left: 0.5714286em;
}

.prose :where(tbody tr):not(:where([class~="not-prose"] *)) {
  border-bottom-width: 1px;
  border-bottom-color: var(--en-prose-td-borders);
}

.prose :where(tbody tr:last-child):not(:where([class~="not-prose"] *)) {
  border-bottom-width: 0;
}

.prose :where(tbody td):not(:where([class~="not-prose"] *)) {
  vertical-align: baseline;
}

.prose :where(tfoot):not(:where([class~="not-prose"] *)) {
  border-top-width: 1px;
  border-top-color: var(--en-prose-th-borders);
}

.prose :where(tfoot td):not(:where([class~="not-prose"] *)) {
  vertical-align: top;
}

.prose :where(p):not(:where([class~="not-prose"] *)) {
  margin-top: 1.25em;
  margin-bottom: 1.25em;
}

.prose :where(video):not(:where([class~="not-prose"] *)) {
  margin-top: 2em;
  margin-bottom: 2em;
}

.prose :where(figure):not(:where([class~="not-prose"] *)) {
  margin-top: 2em;
  margin-bottom: 2em;
}

.prose :where(li):not(:where([class~="not-prose"] *)) {
  margin-top: 0.5em;
  margin-bottom: 0.5em;
}

.prose :where(ol > li):not(:where([class~="not-prose"] *)) {
  padding-left: 0.375em;
}

.prose :where(ul > li):not(:where([class~="not-prose"] *)) {
  padding-left: 0.375em;
}

.prose :where(.prose > ul > li p):not(:where([class~="not-prose"] *)) {
  margin-top: 0.75em;
  margin-bottom: 0.75em;
}

.prose :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.25em;
}

.prose :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.25em;
}

.prose :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.25em;
}

.prose :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.25em;
}

.prose :where(ul ul, ul ol, ol ul, ol ol):not(:where([class~="not-prose"] *)) {
  margin-top: 0.75em;
  margin-bottom: 0.75em;
}

.prose :where(hr + *):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
}

.prose :where(h2 + *):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
}

.prose :where(h3 + *):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
}

.prose :where(h4 + *):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
}

.prose :where(thead th:first-child):not(:where([class~="not-prose"] *)) {
  padding-left: 0;
}

.prose :where(thead th:last-child):not(:where([class~="not-prose"] *)) {
  padding-right: 0;
}

.prose :where(tbody td, tfoot td):not(:where([class~="not-prose"] *)) {
  padding-top: 0.5714286em;
  padding-right: 0.5714286em;
  padding-bottom: 0.5714286em;
  padding-left: 0.5714286em;
}

.prose :where(tbody td:first-child, tfoot td:first-child):not(:where([class~="not-prose"] *)) {
  padding-left: 0;
}

.prose :where(tbody td:last-child, tfoot td:last-child):not(:where([class~="not-prose"] *)) {
  padding-right: 0;
}

.prose :where(:not(kbd) > kbd):not(:where([class~="not-prose"] *)) {
  background-color: var(--en-prose-kbd-bg);
  color: var(--en-prose-kbd-text);
  padding: 0.2em 0.3em;
  border-radius: 0.4em;
}

.prose :where(.prose > :first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
}

.prose :where(.prose > :last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 0;
}

.prose {
  --en-prose-body: #374151;
  --en-prose-headings: #111827;
  --en-prose-lead: #4b5563;
  --en-prose-links: #111827;
  --en-prose-bold: #111827;
  --en-prose-counters: #6b7280;
  --en-prose-bullets: #d1d5db;
  --en-prose-hr: #e5e7eb;
  --en-prose-quotes: #111827;
  --en-prose-quote-borders: #e5e7eb;
  --en-prose-captions: #6b7280;
  --en-prose-code: #111827;
  --en-prose-pre-code: #e5e7eb;
  --en-prose-pre-bg: #1f2937;
  --en-prose-th-borders: #d1d5db;
  --en-prose-td-borders: #e5e7eb;
  --en-prose-kbd-text: #e5e7eb;
  --en-prose-kbd-bg: #1f2937;
  --en-prose-invert-body: #d1d5db;
  --en-prose-invert-headings: #fff;
  --en-prose-invert-lead: #9ca3af;
  --en-prose-invert-links: #fff;
  --en-prose-invert-bold: #fff;
  --en-prose-invert-counters: #9ca3af;
  --en-prose-invert-bullets: #4b5563;
  --en-prose-invert-hr: #374151;
  --en-prose-invert-quotes: #f3f4f6;
  --en-prose-invert-quote-borders: #374151;
  --en-prose-invert-captions: #9ca3af;
  --en-prose-invert-code: #fff;
  --en-prose-invert-pre-code: #d1d5db;
  --en-prose-invert-pre-bg: rgb(0 0 0 / 50%);
  --en-prose-invert-th-borders: #4b5563;
  --en-prose-invert-td-borders: #374151;
  --en-prose-invert-kbd-text: #d1d5db;
  --en-prose-invert-kbd-bg: rgb(0 0 0 / 50%);
  font-size: 1rem;
  line-height: 1.75;
}

.prose-sm :where(.prose > ul > li p):not(:where([class~="not-prose"] *)) {
  margin-top: 0.5714286em;
  margin-bottom: 0.5714286em;
}

.prose-sm :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.1428571em;
}

.prose-sm :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.1428571em;
}

.prose-sm :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.1428571em;
}

.prose-sm :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.1428571em;
}

.prose-sm :where(.prose > :first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
}

.prose-sm :where(.prose > :last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 0;
}

.prose-base :where(.prose > ul > li p):not(:where([class~="not-prose"] *)) {
  margin-top: 0.75em;
  margin-bottom: 0.75em;
}

.prose-base :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.25em;
}

.prose-base :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.25em;
}

.prose-base :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.25em;
}

.prose-base :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.25em;
}

.prose-base :where(.prose > :first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
}

.prose-base :where(.prose > :last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 0;
}

.prose-lg :where(.prose > ul > li p):not(:where([class~="not-prose"] *)) {
  margin-top: 0.8888889em;
  margin-bottom: 0.8888889em;
}

.prose-lg :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.3333333em;
}

.prose-lg :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.3333333em;
}

.prose-lg :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.3333333em;
}

.prose-lg :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.3333333em;
}

.prose-lg :where(.prose > :first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
}

.prose-lg :where(.prose > :last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 0;
}

.prose-xl :where(.prose > ul > li p):not(:where([class~="not-prose"] *)) {
  margin-top: 0.8em;
  margin-bottom: 0.8em;
}

.prose-xl :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.2em;
}

.prose-xl :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.2em;
}

.prose-xl :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.2em;
}

.prose-xl :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.2em;
}

.prose-xl :where(.prose > :first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
}

.prose-xl :where(.prose > :last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 0;
}

.prose-2xl :where(.prose > ul > li p):not(:where([class~="not-prose"] *)) {
  margin-top: 0.8333333em;
  margin-bottom: 0.8333333em;
}

.prose-2xl :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.3333333em;
}

.prose-2xl :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.3333333em;
}

.prose-2xl :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 1.3333333em;
}

.prose-2xl :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 1.3333333em;
}

.prose-2xl :where(.prose > :first-child):not(:where([class~="not-prose"] *)) {
  margin-top: 0;
}

.prose-2xl :where(.prose > :last-child):not(:where([class~="not-prose"] *)) {
  margin-bottom: 0;
}"#;

const PROSE_SLATE_CSS: &[&str] = &[
    "--en-prose-body: #334155;",
    "--en-prose-headings: #0f172a;",
    "--en-prose-lead: #475569;",
    "--en-prose-links: #0f172a;",
    "--en-prose-bold: #0f172a;",
    "--en-prose-counters: #64748b;",
    "--en-prose-bullets: #cbd5e1;",
    "--en-prose-hr: #e2e8f0;",
    "--en-prose-quotes: #0f172a;",
    "--en-prose-quote-borders: #e2e8f0;",
    "--en-prose-captions: #64748b;",
    "--en-prose-code: #0f172a;",
    "--en-prose-pre-code: #e2e8f0;",
    "--en-prose-pre-bg: #1e293b;",
    "--en-prose-th-borders: #cbd5e1;",
    "--en-prose-td-borders: #e2e8f0;",
    "--en-prose-kbd-text: #e2e8f0;",
    "--en-prose-kbd-bg: #1e293b;",
    "--en-prose-invert-body: #cbd5e1;",
    "--en-prose-invert-headings: #fff;",
    "--en-prose-invert-lead: #94a3b8;",
    "--en-prose-invert-links: #fff;",
    "--en-prose-invert-bold: #fff;",
    "--en-prose-invert-counters: #94a3b8;",
    "--en-prose-invert-bullets: #475569;",
    "--en-prose-invert-hr: #334155;",
    "--en-prose-invert-quotes: #f1f5f9;",
    "--en-prose-invert-quote-borders: #334155;",
    "--en-prose-invert-captions: #94a3b8;",
    "--en-prose-invert-code: #fff;",
    "--en-prose-invert-pre-code: #cbd5e1;",
    "--en-prose-invert-pre-bg: rgb(0 0 0 / 50%);",
    "--en-prose-invert-th-borders: #475569;",
    "--en-prose-invert-td-borders: #334155;",
    "--en-prose-invert-kbd-text: #cbd5e1;",
    "--en-prose-invert-kbd-bg: rgb(0 0 0 / 50%);",
];

const PROSE_GRAY_CSS: &[&str] = &[
    "--en-prose-body: #374151;",
    "--en-prose-headings: #111827;",
    "--en-prose-lead: #4b5563;",
    "--en-prose-links: #111827;",
    "--en-prose-bold: #111827;",
    "--en-prose-counters: #6b7280;",
    "--en-prose-bullets: #d1d5db;",
    "--en-prose-hr: #e5e7eb;",
    "--en-prose-quotes: #111827;",
    "--en-prose-quote-borders: #e5e7eb;",
    "--en-prose-captions: #6b7280;",
    "--en-prose-code: #111827;",
    "--en-prose-pre-code: #e5e7eb;",
    "--en-prose-pre-bg: #1f2937;",
    "--en-prose-th-borders: #d1d5db;",
    "--en-prose-td-borders: #e5e7eb;",
    "--en-prose-kbd-text: #e5e7eb;",
    "--en-prose-kbd-bg: #1f2937;",
    "--en-prose-invert-body: #d1d5db;",
    "--en-prose-invert-headings: #fff;",
    "--en-prose-invert-lead: #9ca3af;",
    "--en-prose-invert-links: #fff;",
    "--en-prose-invert-bold: #fff;",
    "--en-prose-invert-counters: #9ca3af;",
    "--en-prose-invert-bullets: #4b5563;",
    "--en-prose-invert-hr: #374151;",
    "--en-prose-invert-quotes: #f3f4f6;",
    "--en-prose-invert-quote-borders: #374151;",
    "--en-prose-invert-captions: #9ca3af;",
    "--en-prose-invert-code: #fff;",
    "--en-prose-invert-pre-code: #d1d5db;",
    "--en-prose-invert-pre-bg: rgb(0 0 0 / 50%);",
    "--en-prose-invert-th-borders: #4b5563;",
    "--en-prose-invert-td-borders: #374151;",
    "--en-prose-invert-kbd-text: #d1d5db;",
    "--en-prose-invert-kbd-bg: rgb(0 0 0 / 50%);",
];

const PROSE_ZINC_CSS: &[&str] = &[
    "--en-prose-body: #3f3f46;",
    "--en-prose-headings: #18181b;",
    "--en-prose-lead: #52525b;",
    "--en-prose-links: #18181b;",
    "--en-prose-bold: #18181b;",
    "--en-prose-counters: #71717a;",
    "--en-prose-bullets: #d4d4d8;",
    "--en-prose-hr: #e4e4e7;",
    "--en-prose-quotes: #18181b;",
    "--en-prose-quote-borders: #e4e4e7;",
    "--en-prose-captions: #71717a;",
    "--en-prose-code: #18181b;",
    "--en-prose-pre-code: #e4e4e7;",
    "--en-prose-pre-bg: #27272a;",
    "--en-prose-th-borders: #d4d4d8;",
    "--en-prose-td-borders: #e4e4e7;",
    "--en-prose-kbd-text: #e4e4e7;",
    "--en-prose-kbd-bg: #27272a;",
    "--en-prose-invert-body: #d4d4d8;",
    "--en-prose-invert-headings: #fff;",
    "--en-prose-invert-lead: #a1a1aa;",
    "--en-prose-invert-links: #fff;",
    "--en-prose-invert-bold: #fff;",
    "--en-prose-invert-counters: #a1a1aa;",
    "--en-prose-invert-bullets: #52525b;",
    "--en-prose-invert-hr: #3f3f46;",
    "--en-prose-invert-quotes: #f4f4f5;",
    "--en-prose-invert-quote-borders: #3f3f46;",
    "--en-prose-invert-captions: #a1a1aa;",
    "--en-prose-invert-code: #fff;",
    "--en-prose-invert-pre-code: #d4d4d8;",
    "--en-prose-invert-pre-bg: rgb(0 0 0 / 50%);",
    "--en-prose-invert-th-borders: #52525b;",
    "--en-prose-invert-td-borders: #3f3f46;",
    "--en-prose-invert-kbd-text: #d4d4d8;",
    "--en-prose-invert-kbd-bg: rgb(0 0 0 / 50%);",
];

const PROSE_NEUTRAL_CSS: &[&str] = &[
    "--en-prose-body: #404040;",
    "--en-prose-headings: #171717;",
    "--en-prose-lead: #525252;",
    "--en-prose-links: #171717;",
    "--en-prose-bold: #171717;",
    "--en-prose-counters: #737373;",
    "--en-prose-bullets: #d4d4d4;",
    "--en-prose-hr: #e5e5e5;",
    "--en-prose-quotes: #171717;",
    "--en-prose-quote-borders: #e5e5e5;",
    "--en-prose-captions: #737373;",
    "--en-prose-code: #171717;",
    "--en-prose-pre-code: #e5e5e5;",
    "--en-prose-pre-bg: #262626;",
    "--en-prose-th-borders: #d4d4d4;",
    "--en-prose-td-borders: #e5e5e5;",
    "--en-prose-kbd-text: #e5e5e5;",
    "--en-prose-kbd-bg: #262626;",
    "--en-prose-invert-body: #d4d4d4;",
    "--en-prose-invert-headings: #fff;",
    "--en-prose-invert-lead: #a3a3a3;",
    "--en-prose-invert-links: #fff;",
    "--en-prose-invert-bold: #fff;",
    "--en-prose-invert-counters: #a3a3a3;",
    "--en-prose-invert-bullets: #525252;",
    "--en-prose-invert-hr: #404040;",
    "--en-prose-invert-quotes: #f5f5f5;",
    "--en-prose-invert-quote-borders: #404040;",
    "--en-prose-invert-captions: #a3a3a3;",
    "--en-prose-invert-code: #fff;",
    "--en-prose-invert-pre-code: #d4d4d4;",
    "--en-prose-invert-pre-bg: rgb(0 0 0 / 50%);",
    "--en-prose-invert-th-borders: #525252;",
    "--en-prose-invert-td-borders: #404040;",
    "--en-prose-invert-kbd-text: #d4d4d4;",
    "--en-prose-invert-kbd-bg: #rgb(0 0 0 / 50%);",
];

const PROSE_STONE_CSS: &[&str] = &[
    "--en-prose-body: #44403c;",
    "--en-prose-headings: #1c1917;",
    "--en-prose-lead: #57534e;",
    "--en-prose-links: #1c1917;",
    "--en-prose-bold: #1c1917;",
    "--en-prose-counters: #78716c;",
    "--en-prose-bullets: #d6d3d1;",
    "--en-prose-hr: #e7e5e4;",
    "--en-prose-quotes: #1c1917;",
    "--en-prose-quote-borders: #e7e5e4;",
    "--en-prose-captions: #78716c;",
    "--en-prose-code: #1c1917;",
    "--en-prose-pre-code: #e7e5e4;",
    "--en-prose-pre-bg: #292524;",
    "--en-prose-th-borders: #d6d3d1;",
    "--en-prose-td-borders: #e7e5e4;",
    "--en-prose-kbd-text: #e7e5e4;",
    "--en-prose-kbd-bg: #292524;",
    "--en-prose-invert-body: #d6d3d1;",
    "--en-prose-invert-headings: #fff;",
    "--en-prose-invert-lead: #a8a29e;",
    "--en-prose-invert-links: #fff;",
    "--en-prose-invert-bold: #fff;",
    "--en-prose-invert-counters: #a8a29e;",
    "--en-prose-invert-bullets: #57534e;",
    "--en-prose-invert-hr: #44403c;",
    "--en-prose-invert-quotes: #f5f5f4;",
    "--en-prose-invert-quote-borders: #44403c;",
    "--en-prose-invert-captions: #a8a29e;",
    "--en-prose-invert-code: #fff;",
    "--en-prose-invert-pre-code: #d6d3d1;",
    "--en-prose-invert-pre-bg: rgb(0 0 0 / 50%);",
    "--en-prose-invert-th-borders: #57534e;",
    "--en-prose-invert-td-borders: #44403c;",
    "--en-prose-invert-kbd-text: #d6d3d1;",
    "--en-prose-invert-kbd-bg: rgb(0 0 0 / 50%);",
];

const PROSE_SM_CSS: &[(&str, &str)] = &[
    (r#""#, "font-size: 0.875rem;\nline-height: 1.7142857;"),
    (r#" :where(p):not(:where([class~="not-prose"] *))"#, "margin-top: 1.1428571em;\nmargin-bottom: 1.1428571em;"),
    (r#" :where([class~="lead"]):not(:where([class~="not-prose"] *))"#, "font-size: 1.2857143em;\nline-height: 1.5555556;\nmargin-top: 0.8888889em;\nmargin-bottom: 0.8888889em;"),
    (r#" :where(blockquote):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;\nmargin-bottom: 1.3333333em;\npadding-left: 1.1111111em;"),
    (r#" :where(h1):not(:where([class~="not-prose"] *))"#, "font-size: 2.1428571em;\nmargin-top: 0;\nmargin-bottom: 0.8em;\nline-height: 1.2;"),
    (r#" :where(h2):not(:where([class~="not-prose"] *))"#, "font-size: 1.4285714em;\nmargin-top: 1.6em;\nmargin-bottom: 0.8em;\nline-height: 1.4;"),
    (r#" :where(h3):not(:where([class~="not-prose"] *))"#, "font-size: 1.2857143em;\nmargin-top: 1.5555556em;\nmargin-bottom: 0.4444444em;\nline-height: 1.5555556;"),
    (r#" :where(h4):not(:where([class~="not-prose"] *))"#, "margin-top: 1.4285714em;\nmargin-bottom: 0.5714286em;\nline-height: 1.4285714;"),
    (r#" :where(img):not(:where([class~="not-prose"] *))"#, "margin-top: 1.7142857em;\nmargin-bottom: 1.7142857em;"),
    (r#" :where(video):not(:where([class~="not-prose"] *))"#, "margin-top: 1.7142857em;\nmargin-bottom: 1.7142857em;"),
    (r#" :where(figure):not(:where([class~="not-prose"] *))"#, "margin-top: 1.7142857em;\nmargin-bottom: 1.7142857em;"),
    (r#" :where(figure > *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;\nmargin-bottom: 0;"),
    (r#" :where(figcaption):not(:where([class~="not-prose"] *))"#, "font-size: 0.8571429em;\nline-height: 1.3333333;\nmargin-top: 0.6666667em;"),
    (r#" :where(code):not(:where([class~="not-prose"] *))"#, "font-size: 0.8571429em;"),
    (r#" :where(h2 code):not(:where([class~="not-prose"] *))"#, "font-size: 0.9em;"),
    (r#" :where(h3 code):not(:where([class~="not-prose"] *))"#, "font-size: 0.8888889em;"),
    (r#" :where(pre):not(:where([class~="not-prose"] *))"#, "font-size: 0.8571429em;\nline-height: 1.6666667;\nmargin-top: 1.6666667em;\nmargin-bottom: 1.6666667em;\nborder-radius: 0.25rem;\npadding-top: 0.6666667em;\npadding-right: 1em;\npadding-bottom: 0.6666667em;\npadding-left: 1em;"),
    (r#" :where(ol):not(:where([class~="not-prose"] *))"#, "margin-top: 1.1428571em;\nmargin-bottom: 1.1428571em;\npadding-left: 1.5714286em;"),
    (r#" :where(ul):not(:where([class~="not-prose"] *))"#, "margin-top: 1.1428571em;\nmargin-bottom: 1.1428571em;\npadding-left: 1.5714286em;"),
    (r#" :where(li):not(:where([class~="not-prose"] *))"#, "margin-top: 0.2857143em;\nmargin-bottom: 0.2857143em;"),
    (r#" :where(ol > li):not(:where([class~="not-prose"] *))"#, "padding-left: 0.4285714em;"),
    (r#" :where(ul > li):not(:where([class~="not-prose"] *))"#, "padding-left: 0.4285714em;"),
    (r#" :where(.prose > ul > li p):not(:where([class~="not-prose"] *))"#, "margin-top: 0.5714286em;\nmargin-bottom: 0.5714286em;"),
    (r#" :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 1.1428571em;"),
    (r#" :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 1.1428571em;"),
    (r#" :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 1.1428571em;"),
    (r#" :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 1.1428571em;"),
    (r#" :where(ul ul, ul ol, ol ul, ol ol):not(:where([class~="not-prose"] *))"#, "margin-top: 0.5714286em;\nmargin-bottom: 0.5714286em;"),
    (r#" :where(hr):not(:where([class~="not-prose"] *))"#, "margin-top: 2.8571429em;\nmargin-bottom: 2.8571429em;"),
    (r#" :where(hr + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h2 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h3 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h4 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(table):not(:where([class~="not-prose"] *))"#, "font-size: 0.8571429em;\nline-height: 1.5;"),
    (r#" :where(thead th):not(:where([class~="not-prose"] *))"#, "padding-right: 1em;\npadding-bottom: 0.6666667em;\npadding-left: 1em;"),
    (r#" :where(thead th:first-child):not(:where([class~="not-prose"] *))"#, "padding-left: 0;"),
    (r#" :where(thead th:last-child):not(:where([class~="not-prose"] *))"#, "padding-right: 0;"),
    (r#" :where(tbody td, tfoot td):not(:where([class~="not-prose"] *))"#, "padding-top: 0.6666667em;\npadding-right: 1em;\npadding-bottom: 0.6666667em;\npadding-left: 1em;"),
    (r#" :where(tbody td:first-child, tfoot td:first-child):not(:where([class~="not-prose"] *))"#, "padding-left: 0;"),
    (r#" :where(tbody td:last-child, tfoot td:last-child):not(:where([class~="not-prose"] *))"#, "padding-right: 0;"),
    (r#" :where(:not(kbd) > kbd):not(:where([class~="not-prose"] *))"#, "background-color: var(--en-prose-kbd-bg);\ncolor: var(--en-prose-kbd-text);\npadding: 0.1em 0.2em;\nborder-radius: 0.4em;"),
    (r#" :where(.prose > :first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(.prose > :last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 0;"),
];

const PROSE_BASE_CSS: &[(&str, &str)] = &[
    (r#""#, "font-size: 1rem;\nline-height: 1.75;"),
    (r#" :where(p):not(:where([class~="not-prose"] *))"#, "margin-top: 1.25em;\nmargin-bottom: 1.25em;"),
    (r#" :where([class~="lead"]):not(:where([class~="not-prose"] *))"#, "font-size: 1.25em;\nline-height: 1.6;\nmargin-top: 1.2em;\nmargin-bottom: 1.2em;"),
    (r#" :where(blockquote):not(:where([class~="not-prose"] *))"#, "margin-top: 1.6em;\nmargin-bottom: 1.6em;\npadding-left: 1em;"),
    (r#" :where(h1):not(:where([class~="not-prose"] *))"#, "font-size: 2.25em;\nmargin-top: 0;\nmargin-bottom: 0.8888889em;\nline-height: 1.1111111;"),
    (r#" :where(h2):not(:where([class~="not-prose"] *))"#, "font-size: 1.5em;\nmargin-top: 2em;\nmargin-bottom: 1em;\nline-height: 1.3333333;"),
    (r#" :where(h3):not(:where([class~="not-prose"] *))"#, "font-size: 1.25em;\nmargin-top: 1.6em;\nmargin-bottom: 0.6em;\nline-height: 1.6;"),
    (r#" :where(h4):not(:where([class~="not-prose"] *))"#, "margin-top: 1.5em;\nmargin-bottom: 0.5em;\nline-height: 1.5;"),
    (r#" :where(img):not(:where([class~="not-prose"] *))"#, "margin-top: 2em;\nmargin-bottom: 2em;"),
    (r#" :where(video):not(:where([class~="not-prose"] *))"#, "margin-top: 2em;\nmargin-bottom: 2em;"),
    (r#" :where(figure):not(:where([class~="not-prose"] *))"#, "margin-top: 2em;\nmargin-bottom: 2em;"),
    (r#" :where(figure > *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;\nmargin-bottom: 0;"),
    (r#" :where(figcaption):not(:where([class~="not-prose"] *))"#, "font-size: 0.875em;\nline-height: 1.4285714;\nmargin-top: 0.8571429em;"),
    (r#" :where(code):not(:where([class~="not-prose"] *))"#, "font-size: 0.875em;"),
    (r#" :where(h2 code):not(:where([class~="not-prose"] *))"#, "font-size: 0.875em;"),
    (r#" :where(h3 code):not(:where([class~="not-prose"] *))"#, "font-size: 0.9em;"),
    (r#" :where(pre):not(:where([class~="not-prose"] *))"#, "font-size: 0.875em;\nline-height: 1.7142857;\nmargin-top: 1.7142857em;\nmargin-bottom: 1.7142857em;\nborder-radius: 0.375rem;\npadding-top: 0.8571429em;\npadding-right: 1.1428571em;\npadding-bottom: 0.8571429em;\npadding-left: 1.1428571em;"),
    (r#" :where(ol):not(:where([class~="not-prose"] *))"#, "margin-top: 1.25em;\nmargin-bottom: 1.25em;\npadding-left: 1.625em;"),
    (r#" :where(ul):not(:where([class~="not-prose"] *))"#, "margin-top: 1.25em;\nmargin-bottom: 1.25em;\npadding-left: 1.625em;"),
    (r#" :where(li):not(:where([class~="not-prose"] *))"#, "margin-top: 0.5em;\nmargin-bottom: 0.5em;"),
    (r#" :where(ol > li):not(:where([class~="not-prose"] *))"#, "padding-left: 0.375em;"),
    (r#" :where(ul > li):not(:where([class~="not-prose"] *))"#, "padding-left: 0.375em;"),
    (r#" :where(.prose > ul > li p):not(:where([class~="not-prose"] *))"#, "margin-top: 0.75em;\nmargin-bottom: 0.75em;"),
    (r#" :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 1.25em;"),
    (r#" :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 1.25em;"),
    (r#" :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 1.25em;"),
    (r#" :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 1.25em;"),
    (r#" :where(ul ul, ul ol, ol ul, ol ol):not(:where([class~="not-prose"] *))"#, "margin-top: 0.75em;\nmargin-bottom: 0.75em;"),
    (r#" :where(hr):not(:where([class~="not-prose"] *))"#, "margin-top: 3em;\nmargin-bottom: 3em;"),
    (r#" :where(hr + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h2 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h3 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h4 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(table):not(:where([class~="not-prose"] *))"#, "font-size: 0.875em;\nline-height: 1.7142857;"),
    (r#" :where(thead th):not(:where([class~="not-prose"] *))"#, "padding-right: 0.5714286em;\npadding-bottom: 0.5714286em;\npadding-left: 0.5714286em;"),
    (r#" :where(thead th:first-child):not(:where([class~="not-prose"] *))"#, "padding-left: 0;"),
    (r#" :where(thead th:last-child):not(:where([class~="not-prose"] *))"#, "padding-right: 0;"),
    (r#" :where(tbody td, tfoot td):not(:where([class~="not-prose"] *))"#, "padding-top: 0.5714286em;\npadding-right: 0.5714286em;\npadding-bottom: 0.5714286em;\npadding-left: 0.5714286em;"),
    (r#" :where(tbody td:first-child, tfoot td:first-child):not(:where([class~="not-prose"] *))"#, "padding-left: 0;"),
    (r#" :where(tbody td:last-child, tfoot td:last-child):not(:where([class~="not-prose"] *))"#, "padding-right: 0;"),
    (r#" :where(:not(kbd) > kbd):not(:where([class~="not-prose"] *))"#, "background-color: var(--en-prose-kbd-bg);\ncolor: var(--en-prose-kbd-text);\npadding: 0.2em 0.3em;\nborder-radius: 0.4em;"),
    (r#" :where(.prose > :first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(.prose > :last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 0;"),
];

const PROSE_LG_CSS: &[(&str, &str)] = &[
    (r#""#, "font-size: 1.125rem;\nline-height: 1.7777778;"),
    (r#" :where(p):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;\nmargin-bottom: 1.3333333em;"),
    (r#" :where([class~="lead"]):not(:where([class~="not-prose"] *))"#, "font-size: 1.2222222em;\nline-height: 1.4545455;\nmargin-top: 1.0909091em;\nmargin-bottom: 1.0909091em;"),
    (r#" :where(blockquote):not(:where([class~="not-prose"] *))"#, "margin-top: 1.6666667em;\nmargin-bottom: 1.6666667em;\npadding-left: 1em;"),
    (r#" :where(h1):not(:where([class~="not-prose"] *))"#, "font-size: 2.6666667em;\nmargin-top: 0;\nmargin-bottom: 0.8333333em;\nline-height: 1;"),
    (r#" :where(h2):not(:where([class~="not-prose"] *))"#, "font-size: 1.6666667em;\nmargin-top: 1.8666667em;\nmargin-bottom: 1.0666667em;\nline-height: 1.3333333;"),
    (r#" :where(h3):not(:where([class~="not-prose"] *))"#, "font-size: 1.3333333em;\nmargin-top: 1.6666667em;\nmargin-bottom: 0.6666667em;\nline-height: 1.5;"),
    (r#" :where(h4):not(:where([class~="not-prose"] *))"#, "margin-top: 1.7777778em;\nmargin-bottom: 0.4444444em;\nline-height: 1.5555556;"),
    (r#" :where(img):not(:where([class~="not-prose"] *))"#, "margin-top: 1.7777778em;\nmargin-bottom: 1.7777778em;"),
    (r#" :where(video):not(:where([class~="not-prose"] *))"#, "margin-top: 1.7777778em;\nmargin-bottom: 1.7777778em;"),
    (r#" :where(figure):not(:where([class~="not-prose"] *))"#, "margin-top: 1.7777778em;\nmargin-bottom: 1.7777778em;"),
    (r#" :where(figure > *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;\nmargin-bottom: 0;"),
    (r#" :where(figcaption):not(:where([class~="not-prose"] *))"#, "font-size: 0.8888889em;\nline-height: 1.5;\nmargin-top: 1em;"),
    (r#" :where(code):not(:where([class~="not-prose"] *))"#, "font-size: 0.8888889em;"),
    (r#" :where(h2 code):not(:where([class~="not-prose"] *))"#, "font-size: 0.8666667em;"),
    (r#" :where(h3 code):not(:where([class~="not-prose"] *))"#, "font-size: 0.875em;"),
    (r#" :where(pre):not(:where([class~="not-prose"] *))"#, "font-size: 0.8888889em;\nline-height: 1.75;\nmargin-top: 2em;\nmargin-bottom: 2em;\nborder-radius: 0.375rem;\npadding-top: 1em;\npadding-right: 1.5em;\npadding-bottom: 1em;\npadding-left: 1.5em;"),
    (r#" :where(ol):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;\nmargin-bottom: 1.3333333em;\npadding-left: 1.5555556em;"),
    (r#" :where(ul):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;\nmargin-bottom: 1.3333333em;\npadding-left: 1.5555556em;"),
    (r#" :where(li):not(:where([class~="not-prose"] *))"#, "margin-top: 0.6666667em;\nmargin-bottom: 0.6666667em;"),
    (r#" :where(ol > li):not(:where([class~="not-prose"] *))"#, "padding-left: 0.4444444em;"),
    (r#" :where(ul > li):not(:where([class~="not-prose"] *))"#, "padding-left: 0.4444444em;"),
    (r#" :where(.prose > ul > li p):not(:where([class~="not-prose"] *))"#, "margin-top: 0.8888889em;\nmargin-bottom: 0.8888889em;"),
    (r#" :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;"),
    (r#" :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 1.3333333em;"),
    (r#" :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;"),
    (r#" :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 1.3333333em;"),
    (r#" :where(ul ul, ul ol, ol ul, ol ol):not(:where([class~="not-prose"] *))"#, "margin-top: 0.8888889em;\nmargin-bottom: 0.8888889em;"),
    (r#" :where(hr):not(:where([class~="not-prose"] *))"#, "margin-top: 3.1111111em;\nmargin-bottom: 3.1111111em;"),
    (r#" :where(hr + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h2 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h3 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h4 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(table):not(:where([class~="not-prose"] *))"#, "font-size: 0.8888889em;\nline-height: 1.5;"),
    (r#" :where(thead th):not(:where([class~="not-prose"] *))"#, "padding-right: 0.75em;\npadding-bottom: 0.75em;\npadding-left: 0.75em;"),
    (r#" :where(thead th:first-child):not(:where([class~="not-prose"] *))"#, "padding-left: 0;"),
    (r#" :where(thead th:last-child):not(:where([class~="not-prose"] *))"#, "padding-right: 0;"),
    (r#" :where(tbody td, tfoot td):not(:where([class~="not-prose"] *))"#, "padding-top: 0.75em;\npadding-right: 0.75em;\npadding-bottom: 0.75em;\npadding-left: 0.75em;"),
    (r#" :where(tbody td:first-child, tfoot td:first-child):not(:where([class~="not-prose"] *))"#, "padding-left: 0;"),
    (r#" :where(tbody td:last-child, tfoot td:last-child):not(:where([class~="not-prose"] *))"#, "padding-right: 0;"),
    (r#" :where(:not(kbd) > kbd):not(:where([class~="not-prose"] *))"#, "background-color: var(--en-prose-kbd-bg);\ncolor: var(--en-prose-kbd-text);\npadding: 0.3em 0.4em;\nborder-radius: 0.4em;"),
    (r#" :where(.prose > :first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(.prose > :last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 0;"),
];

const PROSE_XL_CSS: &[(&str, &str)] = &[
    (r#""#, "font-size: 1.25rem;\nline-height: 1.8;"),
    (r#" :where(p):not(:where([class~="not-prose"] *))"#, "margin-top: 1.2em;\nmargin-bottom: 1.2em;"),
    (r#" :where([class~="lead"]):not(:where([class~="not-prose"] *))"#, "font-size: 1.2em;\nline-height: 1.5;\nmargin-top: 1em;\nmargin-bottom: 1em;"),
    (r#" :where(blockquote):not(:where([class~="not-prose"] *))"#, "margin-top: 1.6em;\nmargin-bottom: 1.6em;\npadding-left: 1.0666667em;"),
    (r#" :where(h1):not(:where([class~="not-prose"] *))"#, "font-size: 2.8em;\nmargin-top: 0;\nmargin-bottom: 0.8571429em;\nline-height: 1;"),
    (r#" :where(h2):not(:where([class~="not-prose"] *))"#, "font-size: 1.8em;\nmargin-top: 1.5555556em;\nmargin-bottom: 0.8888889em;\nline-height: 1.1111111;"),
    (r#" :where(h3):not(:where([class~="not-prose"] *))"#, "font-size: 1.5em;\nmargin-top: 1.6em;\nmargin-bottom: 0.6666667em;\nline-height: 1.3333333;"),
    (r#" :where(h4):not(:where([class~="not-prose"] *))"#, "margin-top: 1.8em;\nmargin-bottom: 0.6em;\nline-height: 1.6;"),
    (r#" :where(img):not(:where([class~="not-prose"] *))"#, "margin-top: 2em;\nmargin-bottom: 2em;"),
    (r#" :where(video):not(:where([class~="not-prose"] *))"#, "margin-top: 2em;\nmargin-bottom: 2em;"),
    (r#" :where(figure):not(:where([class~="not-prose"] *))"#, "margin-top: 2em;\nmargin-bottom: 2em;"),
    (r#" :where(figure > *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;\nmargin-bottom: 0;"),
    (r#" :where(figcaption):not(:where([class~="not-prose"] *))"#, "font-size: 0.9em;\nline-height: 1.5555556;\nmargin-top: 1em;"),
    (r#" :where(code):not(:where([class~="not-prose"] *))"#, "font-size: 0.9em;"),
    (r#" :where(h2 code):not(:where([class~="not-prose"] *))"#, "font-size: 0.8611111em;"),
    (r#" :where(h3 code):not(:where([class~="not-prose"] *))"#, "font-size: 0.9em;"),
    (r#" :where(pre):not(:where([class~="not-prose"] *))"#, "font-size: 0.9em;\nline-height: 1.7777778;\nmargin-top: 2em;\nmargin-bottom: 2em;\nborder-radius: 0.5rem;\npadding-top: 1.1111111em;\npadding-right: 1.3333333em;\npadding-bottom: 1.1111111em;\npadding-left: 1.3333333em;"),
    (r#" :where(ol):not(:where([class~="not-prose"] *))"#, "margin-top: 1.2em;\nmargin-bottom: 1.2em;\npadding-left: 1.6em;"),
    (r#" :where(ul):not(:where([class~="not-prose"] *))"#, "margin-top: 1.2em;\nmargin-bottom: 1.2em;\npadding-left: 1.6em;"),
    (r#" :where(li):not(:where([class~="not-prose"] *))"#, "margin-top: 0.6em;\nmargin-bottom: 0.6em;"),
    (r#" :where(ol > li):not(:where([class~="not-prose"] *))"#, "padding-left: 0.4em;"),
    (r#" :where(ul > li):not(:where([class~="not-prose"] *))"#, "padding-left: 0.4em;"),
    (r#" :where(.prose > ul > li p):not(:where([class~="not-prose"] *))"#, "margin-top: 0.8em;\nmargin-bottom: 0.8em;"),
    (r#" :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 1.2em;"),
    (r#" :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 1.2em;"),
    (r#" :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 1.2em;"),
    (r#" :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 1.2em;"),
    (r#" :where(ul ul, ul ol, ol ul, ol ol):not(:where([class~="not-prose"] *))"#, "margin-top: 0.8em;\nmargin-bottom: 0.8em;"),
    (r#" :where(hr):not(:where([class~="not-prose"] *))"#, "margin-top: 2.8em;\nmargin-bottom: 2.8em;"),
    (r#" :where(hr + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h2 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h3 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h4 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(table):not(:where([class~="not-prose"] *))"#, "font-size: 0.9em;\nline-height: 1.5555556;"),
    (r#" :where(thead th):not(:where([class~="not-prose"] *))"#, "padding-right: 0.6666667em;\npadding-bottom: 0.8888889em;\npadding-left: 0.6666667em;"),
    (r#" :where(thead th:first-child):not(:where([class~="not-prose"] *))"#, "padding-left: 0;"),
    (r#" :where(thead th:last-child):not(:where([class~="not-prose"] *))"#, "padding-right: 0;"),
    (r#" :where(tbody td, tfoot td):not(:where([class~="not-prose"] *))"#, "padding-top: 0.8888889em;\npadding-right: 0.6666667em;\npadding-bottom: 0.8888889em;\npadding-left: 0.6666667em;"),
    (r#" :where(tbody td:first-child, tfoot td:first-child):not(:where([class~="not-prose"] *))"#, "padding-left: 0;"),
    (r#" :where(tbody td:last-child, tfoot td:last-child):not(:where([class~="not-prose"] *))"#, "padding-right: 0;"),
    (r#" :where(:not(kbd) > kbd):not(:where([class~="not-prose"] *))"#, "background-color: var(--en-prose-kbd-bg);\ncolor: var(--en-prose-kbd-text);\npadding: 0.4em 0.5em;\nborder-radius: 0.4em;"),
    (r#" :where(.prose > :first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(.prose > :last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 0;"),
];

const PROSE_2XL_CSS: &[(&str, &str)] = &[
    (r#""#, "font-size: 1.5rem;\nline-height: 1.6666667;"),
    (r#" :where(p):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;\nmargin-bottom: 1.3333333em;"),
    (r#" :where([class~="lead"]):not(:where([class~="not-prose"] *))"#, "font-size: 1.25em;\nline-height: 1.4666667;\nmargin-top: 1.0666667em;\nmargin-bottom: 1.0666667em;"),
    (r#" :where(blockquote):not(:where([class~="not-prose"] *))"#, "margin-top: 1.7777778em;\nmargin-bottom: 1.7777778em;\npadding-left: 1.1111111em;"),
    (r#" :where(h1):not(:where([class~="not-prose"] *))"#, "font-size: 2.6666667em;\nmargin-top: 0;\nmargin-bottom: 0.875em;\nline-height: 1;"),
    (r#" :where(h2):not(:where([class~="not-prose"] *))"#, "font-size: 2em;\nmargin-top: 1.5em;\nmargin-bottom: 0.8333333em;\nline-height: 1.0833333;"),
    (r#" :where(h3):not(:where([class~="not-prose"] *))"#, "font-size: 1.5em;\nmargin-top: 1.5555556em;\nmargin-bottom: 0.6666667em;\nline-height: 1.2222222;"),
    (r#" :where(h4):not(:where([class~="not-prose"] *))"#, "margin-top: 1.6666667em;\nmargin-bottom: 0.6666667em;\nline-height: 1.5;"),
    (r#" :where(img):not(:where([class~="not-prose"] *))"#, "margin-top: 2em;\nmargin-bottom: 2em;"),
    (r#" :where(video):not(:where([class~="not-prose"] *))"#, "margin-top: 2em;\nmargin-bottom: 2em;"),
    (r#" :where(figure):not(:where([class~="not-prose"] *))"#, "margin-top: 2em;\nmargin-bottom: 2em;"),
    (r#" :where(figure > *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;\nmargin-bottom: 0;"),
    (r#" :where(figcaption):not(:where([class~="not-prose"] *))"#, "font-size: 0.8333333em;\nline-height: 1.6;\nmargin-top: 1em;"),
    (r#" :where(code):not(:where([class~="not-prose"] *))"#, "font-size: 0.8333333em;"),
    (r#" :where(h2 code):not(:where([class~="not-prose"] *))"#, "font-size: 0.875em;"),
    (r#" :where(h3 code):not(:where([class~="not-prose"] *))"#, "font-size: 0.8888889em;"),
    (r#" :where(pre):not(:where([class~="not-prose"] *))"#, "font-size: 0.8333333em;\nline-height: 1.8;\nmargin-top: 2em;\nmargin-bottom: 2em;\nborder-radius: 0.5rem;\npadding-top: 1.2em;\npadding-right: 1.6em;\npadding-bottom: 1.2em;\npadding-left: 1.6em;"),
    (r#" :where(ol):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;\nmargin-bottom: 1.3333333em;\npadding-left: 1.5833333em;"),
    (r#" :where(ul):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;\nmargin-bottom: 1.3333333em;\npadding-left: 1.5833333em;"),
    (r#" :where(li):not(:where([class~="not-prose"] *))"#, "margin-top: 0.5em;\nmargin-bottom: 0.5em;"),
    (r#" :where(ol > li):not(:where([class~="not-prose"] *))"#, "padding-left: 0.4166667em;"),
    (r#" :where(ul > li):not(:where([class~="not-prose"] *))"#, "padding-left: 0.4166667em;"),
    (r#" :where(.prose > ul > li p):not(:where([class~="not-prose"] *))"#, "margin-top: 0.8333333em;\nmargin-bottom: 0.8333333em;"),
    (r#" :where(.prose > ul > li > *:first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;"),
    (r#" :where(.prose > ul > li > *:last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 1.3333333em;"),
    (r#" :where(.prose > ol > li > *:first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 1.3333333em;"),
    (r#" :where(.prose > ol > li > *:last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 1.3333333em;"),
    (r#" :where(ul ul, ul ol, ol ul, ol ol):not(:where([class~="not-prose"] *))"#, "margin-top: 0.6666667em;\nmargin-bottom: 0.6666667em;"),
    (r#" :where(hr):not(:where([class~="not-prose"] *))"#, "margin-top: 3em;\nmargin-bottom: 3em;"),
    (r#" :where(hr + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h2 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h3 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(h4 + *):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(table):not(:where([class~="not-prose"] *))"#, "font-size: 0.8333333em;\nline-height: 1.4;"),
    (r#" :where(thead th):not(:where([class~="not-prose"] *))"#, "padding-right: 0.6em;\npadding-bottom: 0.8em;\npadding-left: 0.6em;"),
    (r#" :where(thead th:first-child):not(:where([class~="not-prose"] *))"#, "padding-left: 0;"),
    (r#" :where(thead th:last-child):not(:where([class~="not-prose"] *))"#, "padding-right: 0;"),
    (r#" :where(tbody td, tfoot td):not(:where([class~="not-prose"] *))"#, "padding-top: 0.8em;\npadding-right: 0.6em;\npadding-bottom: 0.8em;\npadding-left: 0.6em;"),
    (r#" :where(tbody td:first-child, tfoot td:first-child):not(:where([class~="not-prose"] *))"#, "padding-left: 0;"),
    (r#" :where(tbody td:last-child, tfoot td:last-child):not(:where([class~="not-prose"] *))"#, "padding-right: 0;"),
    (r#" :where(:not(kbd) > kbd):not(:where([class~="not-prose"] *))"#, "background-color: var(--en-prose-kbd-bg);\ncolor: var(--en-prose-kbd-text);\npadding: 0.4em 0.5em;\nborder-radius: 0.4em;"),
    (r#" :where(.prose > :first-child):not(:where([class~="not-prose"] *))"#, "margin-top: 0;"),
    (r#" :where(.prose > :last-child):not(:where([class~="not-prose"] *))"#, "margin-bottom: 0;"),
];

const PROSE_INVERT_CSS: &str = "--en-prose-body: var(--en-prose-invert-body);
--en-prose-headings: var(--en-prose-invert-headings);
--en-prose-lead: var(--en-prose-invert-lead);
--en-prose-links: var(--en-prose-invert-links);
--en-prose-bold: var(--en-prose-invert-bold);
--en-prose-counters: var(--en-prose-invert-counters);
--en-prose-bullets: var(--en-prose-invert-bullets);
--en-prose-hr: var(--en-prose-invert-hr);
--en-prose-quotes: var(--en-prose-invert-quotes);
--en-prose-quote-borders: var(--en-prose-invert-quote-borders);
--en-prose-captions: var(--en-prose-invert-captions);
--en-prose-code: var(--en-prose-invert-code);
--en-prose-pre-code: var(--en-prose-invert-pre-code);
--en-prose-pre-bg: var(--en-prose-invert-pre-bg);
--en-prose-th-borders: var(--en-prose-invert-th-borders);
--en-prose-td-borders: var(--en-prose-invert-td-borders);
--en-prose-kbd-text: var(--en-prose-invert-kbd-text);
--en-prose-kbd-bg: var(--en-prose-invert-kbd-bg);";

const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "prose" => &[],
        "prose-invert" => &[],
        "prose-gray" => PROSE_GRAY_CSS,
        "prose-slate" => PROSE_SLATE_CSS,
        "prose-zinc" => PROSE_ZINC_CSS,
        "prose-neutral" => PROSE_NEUTRAL_CSS,
        "prose-stone" => PROSE_STONE_CSS,
    },
})
.extra_css(phf_map! {
    "prose" => PROSE_DEFAULT_CSS,
    "prose-invert" => PROSE_INVERT_CSS,
});

const PLUGIN_CUSTOMIZATION: StaticPlugin = Plugin::new(PluginKind::Functional {
    namespace: "prose",
    can_handle: |context| {
        matches!(context.modifier, Modifier::Builtin { value, .. } if [
                "", "sm", "base", "lg", "xl", "2xl", "gray", "slate", "zinc", "neutral", "stone",
                "invert",
            ]
            .contains(value))
    },
    handle: |context| {
        let css_rules = match context.modifier {
            Modifier::Builtin { value: "sm", .. } => PROSE_SM_CSS,
            Modifier::Builtin { value: "base", .. } => PROSE_BASE_CSS,
            Modifier::Builtin { value: "lg", .. } => PROSE_LG_CSS,
            Modifier::Builtin { value: "xl", .. } => PROSE_XL_CSS,
            Modifier::Builtin { value: "2xl", .. } => PROSE_2XL_CSS,
            _ => return,
        };

        generate_at_rules(context, |context| {
            css_rules.iter().for_each(|rule| {
                generate_class(
                    context,
                    |context| context.buffer.lines(rule.1.lines()),
                    rule.0,
                )
            })
        });
    },
});

pub fn register(config: &mut Config) {
    for (name, selector) in [
        ("headings", Some(":where(h1, h2, h3, h4, h5, h6, th)")),
        ("h1", None),
        ("h2", None),
        ("h3", None),
        ("h4", None),
        ("h5", None),
        ("h6", None),
        ("p", None),
        ("a", None),
        ("blockquote", None),
        ("figure", None),
        ("figcaption", None),
        ("strong", None),
        ("em", None),
        ("code", None),
        ("pre", None),
        ("ol", None),
        ("ul", None),
        ("li", None),
        ("table", None),
        ("thead", None),
        ("tr", None),
        ("th", None),
        ("td", None),
        ("kbd", None),
        ("img", None),
        ("video", None),
        ("hr", None),
        ("lead", Some(r#"[class~="lead"]"#)),
    ] {
        config.register_variant(
            format!("prose-{name}"),
            Variant::new(
                config.last_variant_order(),
                format!(
                    r#"& :is({}:not(:where([class~="not-prose"] *)))"#,
                    if let Some(selector) = selector {
                        Cow::from(selector)
                    } else {
                        Cow::from(format!(":where({name})"))
                    }
                ),
            ),
        );
    }

    config.register_plugin(&PLUGIN);
    config.register_plugin(&PLUGIN_CUSTOMIZATION);
}

#[cfg(test)]
mod tests {
    use encre_css::Config;

    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn tailwind_lorem_ipsum() {
        let content = fs::read_to_string("tests/fixtures/tailwind-lorem-ipsum.html").unwrap();
        let expected = fs::read_to_string("tests/fixtures/tailwind-lorem-ipsum.css").unwrap();

        let mut config = Config::default();
        super::register(&mut config);

        let generated = encre_css::generate([content.as_str()], &config);
        assert_eq!(generated, expected.trim_end());
    }
}
