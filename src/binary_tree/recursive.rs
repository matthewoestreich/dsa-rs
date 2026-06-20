#![allow(dead_code)]

use std::fmt;

pub type Child<T> = Option<Box<Node<T>>>;

#[derive(Default, Debug)]
pub struct Node<T>
where
    T: fmt::Debug,
{
    value: T,
    left: Child<T>,
    right: Child<T>,
}

impl<T> Node<T>
where
    T: fmt::Debug,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn print(&self, prefix: String, is_left: bool) {
        println!("{}{:?}", prefix, self.value);
        let mut new_prefix = prefix;
        new_prefix.push_str(if is_left { "├── " } else { "└── " });

        if let Some(ref left) = self.left {
            left.print(new_prefix.clone(), true);
        }
        if let Some(ref right) = self.right {
            right.print(new_prefix, false);
        }
    }
}

pub fn generate(level: i32, value: i32) -> Child<i32> {
    if level == 0 {
        return None;
    }
    let mut n = Box::new(Node::new(value));
    n.left = generate(level - 1, value * 2);
    n.right = generate(level - 1, value * 2 + 1);
    Some(n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gen_recursive() {
        let tree = generate(3, 1);
        if let Some(t) = tree {
            t.print("".to_string(), false);
        }
    }
}
