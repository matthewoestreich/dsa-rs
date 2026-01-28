use std::cmp::Ordering;

use dsa_rs::{
    binary_tree, heap::Heap, linked_list::singly, priority_queue::PriorityQueue, trie::Trie,
};

fn main() {
    /* Binary Tree */
    let mut bin_tree = binary_tree::generate_symmetrical_iteratively(3);
    binary_tree::print_iteratively(&bin_tree);
    binary_tree::invert_in_place_iteratively(&mut bin_tree);
    println!(" ");
    binary_tree::print_iteratively(&bin_tree);

    /* Trie */
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

    /* Heap */
    let comparator = |a: &i32, b: &i32| -> Ordering {
        let r = a - b;
        if r == 0 {
            Ordering::Equal
        } else if r > 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    };

    let values = vec![50, 80, 30, 90, 60, 40 /*, 20*/];
    let mut heap = Heap::new(comparator, Some(values));
    heap.insert(20);

    /* Priority Queue */
    let mut min_priority_queue = PriorityQueue::new(|a: &i32, b: &i32| b.cmp(a), None);
    for v in [67, 39, 71, 22, 382, 4] {
        min_priority_queue.push(v);
    }
    println!(
        "min_priority_queue = {:?}",
        min_priority_queue.to_sorted_vec()
    );

    let mut max_priority_queue = PriorityQueue::new(|a: &i32, b: &i32| a.cmp(b), None);
    for v in [67, 39, 71, 22, 382, 4] {
        max_priority_queue.push(v);
    }
    println!(
        "max_priority_queue = {:?}",
        max_priority_queue.to_sorted_vec()
    );

    /* Singly linked list */
    let mut singly_linked_list = singly::LinkedList::new(0);
    singly_linked_list.insert_last(1);
    singly_linked_list.insert_last(2);
    // Returns reference
    _ = singly_linked_list.head();
    // Returns reference
    _ = singly_linked_list.tail();
    // Removes head and returns it.
    _ = singly_linked_list.pop_head();
    // Removes taill in returns it.
    _ = singly_linked_list.pop_tail();
    // Removes element at "index" 1 and returns it (zero based indexing)
    _ = singly_linked_list.remove_at(1);
}
