#[derive(Default, Debug)]
pub struct Node {
    value: char,
    children: Vec<Node>,
    word: Option<String>,
}

#[derive(Default, Debug)]
pub struct Trie {
    pub root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for ch in word.chars() {
            let found_node = current_node
                .children
                .iter()
                .enumerate()
                .find(|(_, n)| n.value == ch);

            match found_node {
                Some((index, _)) => current_node = &mut current_node.children[index],
                None => {
                    current_node.children.push(Node {
                        value: ch,
                        children: Vec::new(),
                        word: None,
                    });
                    current_node = current_node.children.last_mut().unwrap();
                }
            }
        }

        current_node.word = Some(word.to_string());
    }

    /// Finds all words that start with prefix
    pub fn find_all_by_prefix(&self, prefix: &str) -> Vec<String> {
        let mut cn = &self.root;

        for ch in prefix.chars() {
            let Some((index, _)) = cn.children.iter().enumerate().find(|(_, n)| n.value == ch)
            else {
                return vec![];
            };
            cn = &cn.children[index];
        }

        Self::extract_words(cn)
    }

    pub fn contains(&self, prefix: &str) -> bool {
        let mut cn = &self.root;

        for ch in prefix.chars() {
            let Some((index, _)) = cn.children.iter().enumerate().find(|(_, n)| n.value == ch)
            else {
                return false;
            };
            cn = &cn.children[index];
        }

        true
    }

    /// Extracts all words starting from a given node.
    fn extract_words(root: &Node) -> Vec<String> {
        let mut extracted = vec![];

        if let Some(word) = root.word.as_ref() {
            extracted.push(word.to_string());
        }

        for child_node in root.children.iter() {
            let mut child_words = Self::extract_words(child_node);
            extracted.append(&mut child_words);
        }

        extracted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        //
        // In order or these test to pass, each word
        // must start with a unique letter
        //
        let mut t_1 = Trie::new();
        t_1.insert("foo");
        assert_eq!(t_1.root.children.len(), 1);
        t_1.insert("bar");
        assert_eq!(t_1.root.children.len(), 2);

        let mut t_2 = Trie::new();
        let bulk = ["bing", "aapple", "car", "door", "zinger"];
        bulk.iter().for_each(|el| t_2.insert(el));
        assert_eq!(t_2.root.children.len(), bulk.len());
    }

    #[test]
    fn test_autocomplete() {
        let mut t = Trie::new();
        let bulk = ["hello", "help", "helicopter", "helipad"];
        bulk.iter().for_each(|word| t.insert(word));
        assert_eq!(t.find_all_by_prefix("h"), bulk);
        assert_eq!(t.find_all_by_prefix("heli"), ["helicopter", "helipad"]);
    }

    #[test]
    fn test_contains() {
        let mut t = Trie::new();
        let bulk = ["hello", "help", "helicopter", "helipad"];
        bulk.iter().for_each(|word| t.insert(word));
        bulk.iter().for_each(|word| assert!(t.contains(word)));
        assert!(!t.contains("zzz"));
        assert!(!t.contains("hz"));
        assert!(t.contains("heli"));
    }
}
