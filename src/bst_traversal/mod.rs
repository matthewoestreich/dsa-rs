#[derive(Debug)]
pub struct BSTNode<T> {
    pub value: T,
    pub left: Option<Box<BSTNode<T>>>,
    pub right: Option<Box<BSTNode<T>>>,
}

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    pub root: Option<Box<BSTNode<T>>>,
}

impl<T> BinarySearchTree<T> {
    pub fn with_n_nodes<F>(n: usize, mut f: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        fn build<T, F>(i: usize, n: usize, f: &mut F) -> Option<Box<BSTNode<T>>>
        where
            F: FnMut(usize) -> T,
        {
            if i >= n {
                return None;
            }

            Some(Box::new(BSTNode {
                value: f(i),
                left: build(2 * i + 1, n, f),
                right: build(2 * i + 2, n, f),
            }))
        }

        Self {
            root: build(0, n, &mut f),
        }
    }

    pub fn inorder(&self) -> Vec<&T> {
        let mut values = Vec::new();

        fn traverse<'a, T>(node: &'a Option<Box<BSTNode<T>>>, values: &mut Vec<&'a T>) {
            if let Some(node) = node {
                traverse(&node.left, values);
                values.push(&node.value);
                traverse(&node.right, values);
            }
        }

        traverse(&self.root, &mut values);
        values
    }

    pub fn preorder(&self) -> Vec<&T> {
        let mut values = Vec::new();

        fn traverse<'a, T>(node: &'a Option<Box<BSTNode<T>>>, values: &mut Vec<&'a T>) {
            if let Some(node) = node {
                values.push(&node.value);
                traverse(&node.left, values);
                traverse(&node.right, values);
            }
        }

        traverse(&self.root, &mut values);
        values
    }

    pub fn postorder(&self) -> Vec<&T> {
        let mut values = Vec::new();

        fn traverse<'a, T>(node: &'a Option<Box<BSTNode<T>>>, values: &mut Vec<&'a T>) {
            if let Some(node) = node {
                traverse(&node.left, values);
                traverse(&node.right, values);
                values.push(&node.value);
            }
        }

        traverse(&self.root, &mut values);
        values
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bst_inorder() {
        let bst = BinarySearchTree::with_n_nodes(3, |i| (i as u32) + 1);
        let expect = vec![&2, &1, &3];
        assert_eq!(expect, bst.inorder());
    }

    #[test]
    fn test_bst_preorder() {
        let bst = BinarySearchTree::with_n_nodes(3, |i| (i as u32) + 1);
        let expect = vec![&1, &2, &3];
        assert_eq!(expect, bst.preorder());
    }

    #[test]
    fn test_bst_postorder() {
        let bst = BinarySearchTree::with_n_nodes(3, |i| (i as u32) + 1);
        let expect = vec![&2, &3, &1];
        assert_eq!(expect, bst.postorder());
    }
}
