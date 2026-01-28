use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display},
};

#[derive(Clone)]
pub struct Heap<T, F>
where
    T: PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    nodes: Vec<T>,
    compare: F,
}

impl<T, F> Debug for Heap<T, F>
where
    T: PartialEq + Eq + Debug,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let leaf = self.leaf();
        f.debug_struct("Heap")
            .field("nodes", &self.nodes)
            .field("leaf", &leaf)
            .finish()
    }
}

impl<T, F> Display for Heap<T, F>
where
    T: PartialEq + Eq + Display,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nodes_len = self.nodes.len();
        write!(f, "Heap {{ nodes: [")?;
        for (i, node) in self.nodes.iter().enumerate() {
            if i < nodes_len - 1 {
                write!(f, "{node}, ")?;
            } else {
                write!(f, "{node}]")?;
            }
        }
        match self.leaf() {
            Some(leaf) => write!(f, ", leaf: Some({leaf})")?,
            None => write!(f, ", leaf: None")?,
        }
        write!(f, " }}")
    }
}

impl<T, F> PartialEq for Heap<T, F>
where
    T: PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes && self.leaf() == other.leaf()
    }
}

impl<T, F> Eq for Heap<T, F>
where
    T: PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
}

/// Immutable iteration.
impl<'a, T, F> IntoIterator for &'a Heap<T, F>
where
    T: PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.iter()
    }
}

/// Consuming iteration.
impl<T, F> IntoIterator for Heap<T, F>
where
    T: PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl<T, F> Heap<T, F>
where
    T: PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    pub fn new(compare: F, values: Option<Vec<T>>) -> Self {
        let mut this = Self {
            compare,
            nodes: values.unwrap_or_default(),
        };
        if !this.is_empty() {
            this.fix();
        }
        this
    }

    /// Pushes a value in the heap.
    pub fn insert(&mut self, value: T) {
        self.nodes.push(value);
        self.heapify_up(self.size() - 1);
    }

    /// Alias for insert.
    pub fn push(&mut self, value: T) {
        self.insert(value);
    }

    /// Removes and returns root node.
    pub fn extract_root(&mut self) -> Option<T> {
        if self.nodes.is_empty() {
            return None;
        }

        self.swap(0, self.size() - 1);
        let root = self.nodes.pop();

        if !self.nodes.is_empty() {
            self.heapify_down(0);
        }

        root
    }

    /// Alias for extract_root.
    pub fn pop(&mut self) -> Option<T> {
        self.extract_root()
    }

    /// Returns a reference to the root node.
    pub fn root(&self) -> Option<&T> {
        self.nodes.first()
    }

    /// Alias for 'root' method.
    pub fn top(&self) -> Option<&T> {
        self.root()
    }

    /// Alias for `root` method.
    pub fn front(&self) -> Option<&T> {
        self.nodes.last()
    }

    /// Returns reference to element with lowest priority.
    pub fn leaf(&self) -> Option<&T> {
        self.nodes.iter().min_by(|a, b| (self.compare)(a, b))
    }

    pub fn clear(&mut self) {
        self.nodes = Vec::new();
    }

    /// Returns number of nodes in heap.
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.nodes.iter()
    }

    /// Use in place of iter_mut.
    /// Temporarily gives mutable access to each node and automatically fixes the heap afterward.
    pub fn for_each_mut<C>(&mut self, callback: C)
    where
        C: FnMut(&mut T),
    {
        self.nodes.iter_mut().for_each(callback);
        self.fix();
    }

    pub fn to_sorted_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut clone = self.clone();
        let mut sorted = Vec::with_capacity(clone.size());
        while let Some(item) = clone.pop() {
            sorted.push(item);
        }
        sorted
    }

    /// Checks if heap is valid.
    pub fn is_valid(&self) -> bool {
        self.is_valid_from(0)
    }

    /// Recursively checks if the heap is valid, starting from specified index.
    fn is_valid_from(&self, index: usize) -> bool {
        let left_child = (index * 2) + 1;
        let right_child = (index * 2) + 2;

        let has_left = left_child < self.size();
        let has_right = right_child < self.size();

        let mut valid_left = true;
        let mut valid_right = true;

        if has_left {
            if self.compare_at(index, left_child) == Ordering::Greater {
                return false;
            }
            valid_left = self.is_valid_from(left_child);
        }

        if has_right {
            if self.compare_at(index, right_child) == Ordering::Greater {
                return false;
            }
            valid_right = self.is_valid_from(right_child);
        }

        valid_left && valid_right
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.nodes.swap(i, j);
    }

    fn compare_at(&self, a: usize, b: usize) -> Ordering {
        (self.compare)(&self.nodes[a], &self.nodes[b])
    }

    fn heapify_up(&mut self, start_index: usize) {
        let mut child = start_index;

        while child > 0 {
            let parent = (child - 1) / 2;
            if self.compare_at(child, parent) == Ordering::Greater {
                self.swap(child, parent);
                child = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, start_index: usize) {
        let mut parent = start_index;

        loop {
            let mut candidate = parent;
            let left_child = (parent * 2) + 1;
            let right_child = (parent * 2) + 2;

            if left_child < self.size()
                && self.compare_at(left_child, candidate) == Ordering::Greater
            {
                candidate = left_child;
            }

            if right_child < self.size()
                && self.compare_at(right_child, candidate) == Ordering::Greater
            {
                candidate = right_child;
            }

            if candidate == parent {
                break;
            }

            self.swap(parent, candidate);
            parent = candidate;
        }
    }

    fn fix(&mut self) {
        // Fix node positions.
        for i in (0..=(self.size() / 2) - 1).rev() {
            self.heapify_down(i);
        }
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Foo {
        id: i32,
    }

    impl Foo {
        fn new(id: i32) -> Self {
            Self { id }
        }
    }

    impl Display for Foo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Foo {{ id: {} }}", self.id)
        }
    }

    #[test]
    fn test_extract_root_on_empty_heap() {
        let mut heap = Heap::new(|a: &i32, b: &i32| -> Ordering { a.cmp(b) }, None);
        heap.clear();
        let root = heap.extract_root();
        assert_eq!(root, None);
    }

    #[test]
    fn test_max_heap() {
        let compare = |a: &i32, b: &i32| a.cmp(b);
        let values = vec![30, 20, 90, 50, 60, 10];
        let max_heap = Heap::new(compare, Some(values));

        let heap_to_sorted = max_heap.to_sorted_vec();
        let expected_sort = vec![90, 60, 50, 30, 20, 10];
        println!("max_heap_to_sorted = {heap_to_sorted:?}\nexpected_sort = {expected_sort:?}");
        assert_eq!(heap_to_sorted, expected_sort);
    }

    #[test]
    fn test_min_heap() {
        let compare = |a: &i32, b: &i32| b.cmp(a);
        let values = vec![30, 20, 90, 50, 60, 10];
        let min_heap = Heap::new(compare, Some(values));

        let heap_to_sorted = min_heap.to_sorted_vec();
        let expected_sort = vec![10, 20, 30, 50, 60, 90];
        println!("min_heap_to_sorted = {heap_to_sorted:?}\nexpected_sort = {expected_sort:?}");
        assert_eq!(heap_to_sorted, expected_sort);
    }

    #[test]
    fn test_min_heap_foo() {
        let mut heap = Heap::<Foo, _>::new(|a, b| b.id.cmp(&a.id), None);
        let values = vec![
            Foo::new(50),
            Foo::new(80),
            Foo::new(30),
            Foo::new(90),
            Foo::new(60),
            Foo::new(40),
            Foo::new(20),
        ];

        // Test insert
        for &value in values.iter() {
            heap.insert(value);
        }
        assert_eq!(
            values.len(),
            heap.size(),
            "expected values.len ({}) to == heap.size() ({})",
            values.len(),
            heap.size()
        );

        // Test sort
        let to_sorted: Vec<_> = heap.to_sorted_vec();
        println!("min_foo_heap_to_sorted = {to_sorted:?}\nfoo_heap = {heap}");
        let mut values_for_sort = values.clone();
        values_for_sort.sort();
        assert_eq!(to_sorted, values_for_sort);

        println!("{heap}");

        // Test root value
        let root_node = heap.root().expect("exist");
        assert_eq!(root_node, &Foo::new(20));

        // Test leaf value
        let leaf_node = heap.leaf().expect("some");
        assert_eq!(leaf_node, &Foo::new(90));

        // Test size
        assert_eq!(heap.size(), values.len());

        // Test is_empty
        assert!(!heap.is_empty());

        // Test clone
        let heap_clone = heap.clone();
        assert!(heap.eq(&heap_clone));

        // Test immutable iteration
        let mut collected: Vec<_> = vec![]; // OR : (&heap).into_iter().copied().collect();
        for &el in heap.iter() {
            collected.push(el);
        }
        assert_eq!(heap.nodes.clone(), collected);
        collected.sort();
        let mut collected_vals = values.clone();
        collected_vals.sort();
        assert_eq!(collected, collected_vals);

        // Test mutable iteration.
        let multiplyer = 10;
        let mut heap_clone_for_mut_iter = heap.clone();
        let mut vals_clone: Vec<_> = values.clone();
        vals_clone.iter_mut().for_each(|v| v.id *= multiplyer);
        heap_clone_for_mut_iter.for_each_mut(|el| el.id *= multiplyer);
        vals_clone.sort();
        let heap_nodes_vec = heap_clone_for_mut_iter.to_sorted_vec();
        assert_eq!(vals_clone, heap_nodes_vec);

        // Test extract_root
        assert_eq!(heap.extract_root().expect("exist"), Foo::new(20));
        assert_eq!(heap.extract_root().expect("exist"), Foo::new(30));
        assert_eq!(heap.extract_root().expect("exist"), Foo::new(40));
        assert_eq!(heap.extract_root().expect("exist"), Foo::new(50));
        assert_eq!(heap.extract_root().expect("exist"), Foo::new(60));
        assert_eq!(heap.extract_root().expect("exist"), Foo::new(80));
        assert_eq!(heap.extract_root().expect("exist"), Foo::new(90));
        assert!(heap.is_empty());
    }
}
