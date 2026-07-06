use std::{borrow::Cow, collections::HashMap};

use crate::{
    Config,
    config::BUILTIN_PLUGINS,
    plugins::{Plugin, PluginKind},
};

#[derive(Debug, Clone)]
pub(crate) struct TrieData {
    pub(crate) has_prefix: bool,
    pub(crate) is_custom: bool,
    pub(crate) order: usize,
    pub(crate) plugin: &'static Plugin,
}

#[derive(Default, Debug)]
struct TrieNode {
    data: Option<Vec<TrieData>>,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
pub(crate) struct Trie {
    root: TrieNode,
}

impl Trie {
    pub(crate) fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub(crate) fn insert(&mut self, word: &str, data: TrieData) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }

        current_node.data.get_or_insert_default().push(data);
    }

    pub(crate) fn common_prefix_search<'a>(
        &self,
        word: &'a str,
    ) -> (Cow<'a, str>, Box<dyn Iterator<Item = &TrieData> + '_>) {
        let mut current_node = &self.root;
        let mut common_prefix = 0;
        let mut last_data = current_node.data.as_ref();
        let chars: Vec<char> = word.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            // Store the data of the last node if a dash (or a slash because the modifier can be
            // extended by a slash) is the next character
            if (*c == '-' || *c == '/') && current_node.data.is_some() {
                common_prefix = i;
                last_data = current_node.data.as_ref();
            }

            match current_node.children.get(&c) {
                Some(node) => {
                    current_node = node;
                }
                None => {
                    return (
                        Cow::Owned(chars.iter().take(common_prefix).collect()),
                        if let Some(data) = last_data {
                            Box::new(data.iter())
                        } else {
                            Box::new(std::iter::empty())
                        },
                    );
                }
            }
        }

        (
            Cow::Borrowed(word),
            if let Some(data) = current_node.data.as_ref() {
                Box::new(data.iter())
            } else {
                Box::new(std::iter::empty())
            },
        )
    }
}

pub(crate) fn build_trie(config: &Config) -> Trie {
    let mut trie = Trie::new();

    for (order, (plugin, is_custom)) in BUILTIN_PLUGINS
        .iter()
        .map(|r| (r, false))
        .enumerate()
        .chain(config.custom_plugins.iter().map(|r| (r, true)).enumerate())
    {
        match &plugin.kind {
            PluginKind::ListCases { cases } => {
                if let Some(prefix) = plugin.list_prefix {
                    trie.insert(
                        prefix,
                        TrieData {
                            has_prefix: true,
                            is_custom,
                            order,
                            plugin: *plugin,
                        },
                    );
                } else {
                    for class in cases.keys() {
                        trie.insert(
                            *class,
                            TrieData {
                                has_prefix: false,
                                is_custom,
                                order,
                                plugin: *plugin,
                            },
                        );
                    }
                }
            }
            PluginKind::ListValues { values, .. } => {
                if let Some(prefix) = plugin.list_prefix {
                    trie.insert(
                        prefix,
                        TrieData {
                            has_prefix: true,
                            is_custom,
                            order,
                            plugin: *plugin,
                        },
                    );
                } else {
                    for class in values.keys() {
                        trie.insert(
                            *class,
                            TrieData {
                                has_prefix: false,
                                is_custom,
                                order,
                                plugin: *plugin,
                            },
                        );
                    }
                }
            }
            PluginKind::Spacing { prefix, .. }
            | PluginKind::Color { prefix, .. }
            | PluginKind::AnyNumber { prefix, .. }
            | PluginKind::Arbitrary { prefix, .. }
            | PluginKind::ArbitraryShadow { prefix, .. } => {
                trie.insert(
                    prefix,
                    TrieData {
                        has_prefix: true,
                        is_custom,
                        order,
                        plugin: *plugin,
                    },
                );
            }
            PluginKind::Functional { class, .. } => {
                trie.insert(
                    class,
                    TrieData {
                        has_prefix: false,
                        is_custom,
                        order,
                        plugin: *plugin,
                    },
                );
            }
        }
    }

    trie
}
