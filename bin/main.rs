use dsa_rs::{
    binary_tree,
    heap::{ComparatorResult, Heap},
    trie::Trie,
};

fn main() {
    // Binary Tree
    let mut bin_tree = binary_tree::generate_symmetrical_iteratively(3);
    binary_tree::print_iteratively(&bin_tree);
    binary_tree::invert_in_place_iteratively(&mut bin_tree);
    println!(" ");
    binary_tree::print_iteratively(&bin_tree);

    // Trie
    let words_for_trie = vec![
        "astronaut",
        "astronomy",
        "astrophysics",
        "microscope",
        "microchip",
        "microbe",
        "translate",
        "transport",
        "transform",
        "lantern",
    ];
    let mut trie = Trie::new();
    words_for_trie.iter().for_each(|w| trie.insert(w));
    let ast_prefix = trie.find_all_by_prefix("ast");
    println!("words that start with 'ast' = {ast_prefix:?}");

    // Heap
    let comparator = |a: &i32, b: &i32| -> ComparatorResult {
        let r = a - b;
        if r == 0 {
            ComparatorResult::Equal
        } else if r > 0 {
            ComparatorResult::Greater
        } else {
            ComparatorResult::Less
        }
    };

    let values = vec![50, 80, 30, 90, 60, 40 /*, 20*/];
    let mut heap = Heap::new(comparator, Some(values));
    heap.insert(20);
}
