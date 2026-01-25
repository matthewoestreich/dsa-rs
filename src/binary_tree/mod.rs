use std::collections::VecDeque;

#[derive(Default, Debug)]
pub struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new_with_value(value: u32) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

// Used for printing.
struct NodeDepth<'a> {
    node: &'a Node,
    depth: usize,
}

/// Generates a symmetrical binary tree iteratively. Does not use recursion.
pub fn generate_symmetrical_iteratively(num_levels: u32) -> Node {
    if num_levels == 0 {
        return Node::default();
    }

    let mut node_count = 1;
    let mut root = Box::new(Node::new_with_value(1));
    let mut curr_level = vec![&mut root];

    for _ in 1..num_levels {
        let mut next_level = vec![];
        for node in curr_level {
            node_count += 1;
            node.left = Some(Box::new(Node::new_with_value(node_count)));
            node_count += 1;
            node.right = Some(Box::new(Node::new_with_value(node_count)));
            next_level.push(node.left.as_mut().expect("just set value"));
            next_level.push(node.right.as_mut().expect("just set value"));
        }
        curr_level = next_level;
    }

    *root
}

/// Iteratively prints a tree. Does not use recursion.
pub fn print_iteratively(root: &Node) {
    let mut stack: Vec<NodeDepth> = Vec::new();
    let mut current = Some(NodeDepth {
        node: root,
        depth: 1,
    });

    while current.is_some() || !stack.is_empty() {
        while let Some(curr) = current {
            let next = curr.node.right.as_ref().map(|n| NodeDepth {
                node: n.as_ref(),
                depth: curr.depth + 1,
            });
            stack.push(curr);
            current = next;
        }

        if let Some(tn) = stack.pop() {
            println!("{:indent$}{}", "", tn.node.value, indent = tn.depth * 2);
            current = tn.node.left.as_ref().map(|n| NodeDepth {
                node: n.as_ref(),
                depth: tn.depth + 1,
            });
        }
    }
}

/// Iteratively inverts a tree in place (modifying tree that is passed in).
/// Does not use recursion.
pub fn invert_in_place_iteratively(root: &mut Node) {
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        std::mem::swap(&mut node.left, &mut node.right);

        if let Some(ref mut left) = node.left {
            queue.push_back(left);
        }
        if let Some(ref mut right) = node.right {
            queue.push_back(right);
        }
    }
}
