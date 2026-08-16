use std::{borrow::Cow, collections::HashMap};

use crate::{
    Config, config::BUILTIN_PLUGINS, plugins::{Arbitrary, Color, CustomPlugin, Functional, ListProperties, ListValues, Number, Plugin, Spacing},
};

#[derive(Debug)]
pub(crate) struct TrieData {
    pub(crate) has_namespace: bool,
    pub(crate) is_custom: bool,
    pub(crate) order: usize,
    pub(crate) plugin: CustomPlugin,
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

            match current_node.children.get(c) {
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

    for (order, (is_custom, plugin)) in BUILTIN_PLUGINS
        .iter()
        .map(|p| (false, p))
        .enumerate()
        .chain(
            config
                .custom_plugins
                .iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    if let CustomPlugin::Static(s) = p {
                        Some((i, (true, s)))
                    } else {
                        None
                    }
                }),
        )
    {
        match &plugin {
            Plugin::ListProperties(ListProperties { namespace, props, .. }) => {
                if let Some(namespace) = namespace {
                    trie.insert(
                        namespace,
                        TrieData {
                            has_namespace: true,
                            is_custom,
                            order,
                            plugin: CustomPlugin::Static(plugin),
                        },
                    );
                } else {
                    for class in props.keys() {
                        trie.insert(
                            class,
                            TrieData {
                                has_namespace: false,
                                is_custom,
                                order,
                                plugin: CustomPlugin::Static(plugin),
                            },
                        );
                    }
                }
            }
            Plugin::ListValues(ListValues { namespace, values, .. }) => {
                if let Some(namespace) = namespace {
                    trie.insert(
                        namespace,
                        TrieData {
                            has_namespace: true,
                            is_custom,
                            order,
                            plugin: CustomPlugin::Static(plugin),
                        },
                    );
                } else {
                    for class in values.keys() {
                        trie.insert(
                            class,
                            TrieData {
                                has_namespace: false,
                                is_custom,
                                order,
                                plugin: CustomPlugin::Static(plugin),
                            },
                        );
                    }
                }
            }
            Plugin::Spacing(Spacing { namespace, .. })
            | Plugin::Color(Color { namespace, .. })
            | Plugin::Number(Number { namespace, .. })
            | Plugin::Arbitrary(Arbitrary { namespace, .. })
            | Plugin::Functional(Functional { namespace, .. }) => {
                trie.insert(
                    namespace,
                    TrieData {
                        has_namespace: true,
                        is_custom,
                        order,
                        plugin: CustomPlugin::Static(plugin),
                    },
                );
            }
        }
    }

    for (order, plugin) in config
        .custom_plugins
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if let CustomPlugin::Dynamic(s) = p {
                Some((i, s))
            } else {
                None
            }
        })
    {
        match &plugin {
            Plugin::ListProperties(ListProperties { namespace, props, .. }) => {
                if let Some(namespace) = &namespace {
                    trie.insert(
                        namespace,
                        TrieData {
                            has_namespace: true,
                            is_custom: false,
                            order,
                            plugin: CustomPlugin::Dynamic(plugin.clone()),
                        },
                    );
                } else {
                    for class in props.keys() {
                        trie.insert(
                            class,
                            TrieData {
                                has_namespace: false,
                                is_custom: false,
                                order,
                                plugin: CustomPlugin::Dynamic(plugin.clone()),
                            },
                        );
                    }
                }
            }
            Plugin::ListValues(ListValues { namespace, values, .. }) => {
                if let Some(namespace) = &namespace {
                    trie.insert(
                        namespace,
                        TrieData {
                            has_namespace: true,
                            is_custom: false,
                            order,
                            plugin: CustomPlugin::Dynamic(plugin.clone()),
                        },
                    );
                } else {
                    for class in values.keys() {
                        trie.insert(
                            class,
                            TrieData {
                                has_namespace: false,
                                is_custom: false,
                                order,
                                plugin: CustomPlugin::Dynamic(plugin.clone()),
                            },
                        );
                    }
                }
            }
            Plugin::Spacing(Spacing { namespace, .. })
            | Plugin::Color(Color { namespace, .. })
            | Plugin::Number(Number { namespace, .. })
            | Plugin::Arbitrary(Arbitrary { namespace, .. })
            | Plugin::Functional(Functional { namespace, .. }) => {
                trie.insert(
                    namespace,
                    TrieData {
                        has_namespace: true,
                        is_custom: false,
                        order,
                        plugin: CustomPlugin::Dynamic(plugin.clone()),
                    },
                );
            }
        }
    }

    trie
}
