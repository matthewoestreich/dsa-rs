use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum ComparatorResult {
    Greater,
    Less,
    Equal,
}

pub struct Heap<T, F>
where
    T: Copy + PartialEq + Eq,
    F: Fn(&T, &T) -> ComparatorResult,
{
    comparator: F,
    pub(crate) nodes: VecDeque<T>,
    pub(crate) leaf: Option<T>,
}

#[allow(dead_code)]
impl<T, F> Heap<T, F>
where
    T: Copy + PartialEq + Eq,
    F: Fn(&T, &T) -> ComparatorResult,
{
    pub fn new(comparator: F, values: Option<Vec<T>>) -> Self {
        let mut this = Self {
            comparator,
            nodes: if let Some(vals) = values {
                VecDeque::from(vals)
            } else {
                VecDeque::new()
            },
            leaf: None,
        };
        if !this.nodes.is_empty() {
            this.fix();
        }
        this
    }

    /// Pushes a value in the heap.
    pub fn insert(&mut self, value: T) {
        self.nodes.push_back(value);
        self.heapify_up(self.size() - 1);
        if self
            .leaf
            .is_none_or(|leaf| (self.comparator)(&value, &leaf) == ComparatorResult::Greater)
        {
            self.leaf = Some(value);
        }
    }

    /// Alias for insert.
    pub fn push(&mut self, value: T) {
        self.insert(value);
    }

    /// Removes and returns root node.
    pub fn extract_root(&mut self) -> Option<T> {
        let root_opt = self.nodes.swap_remove_back(0);
        self.heapify_down(0);

        if let Some(root) = root_opt
            && let Some(leaf) = self.leaf
            && root == leaf
        {
            self.leaf = None;
        }

        root_opt
    }

    /// Alias for extract_root.
    pub fn pop(&mut self) -> Option<T> {
        self.extract_root()
    }

    /// Returns a copy of the root node.
    pub fn root(&self) -> Option<T> {
        self.nodes.front().cloned()
    }

    /// Alias for 'root' method.
    pub fn top(&self) -> Option<T> {
        self.root()
    }

    pub fn clear(&mut self) {
        self.nodes = VecDeque::new();
        self.leaf = None;
    }

    /// Returns number of nodes in heap.
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Sorts heap data in place.
    pub fn sort(&mut self) {
        for i in (0..self.size()).rev() {
            self.swap(0, i);
            self.heapify_down_until(i);
        }
    }

    /// Sorts in place as well as returns copy of sorted data.
    pub fn to_sorted(&mut self) -> Vec<T> {
        self.sort();
        Vec::from(self.nodes.clone())
    }

    /// Checks if heap is valid.
    pub fn is_valid(&self) -> bool {
        self.is_valid_from(0)
    }

    /// Recursively checks if the heap is valid, starting from specified parent index.
    fn is_valid_from(&self, parent_index: usize) -> bool {
        let mut is_valid_left = true;
        let mut is_valid_right = true;

        if self.has_left_child(parent_index) {
            let left_child_index = (parent_index * 2) + 1;
            if self.compare_at(parent_index, left_child_index) == ComparatorResult::Greater {
                return false;
            }
            is_valid_left = self.is_valid_from(left_child_index);
        }

        if self.has_right_child(parent_index) {
            let right_child_index = (parent_index * 2) + 2;
            if self.compare_at(parent_index, right_child_index) == ComparatorResult::Greater {
                return false;
            }
            is_valid_right = self.is_valid_from(right_child_index);
        }

        is_valid_left && is_valid_right
    }

    fn has_left_child(&self, parent_index: usize) -> bool {
        let left_child_index = (parent_index * 2) + 1;
        left_child_index < self.size()
    }

    fn has_right_child(&self, parent_index: usize) -> bool {
        let right_child_index = (parent_index * 2) + 2;
        right_child_index < self.size()
    }

    fn pick_child_of(&self, parent_index: usize) -> Option<usize> {
        let has_left = self.has_left_child(parent_index);
        let has_right = self.has_right_child(parent_index);
        if !has_left && !has_right {
            return None;
        }

        let left_child_index = (parent_index * 2) + 1;
        let right_child_index = (parent_index * 2) + 2;

        if !has_left {
            return Some(right_child_index);
        }
        if !has_right {
            return Some(left_child_index);
        }

        Some(match self.compare_at(left_child_index, right_child_index) {
            ComparatorResult::Greater => right_child_index,
            _ => left_child_index,
        })
    }

    fn pick_child_before(
        &self,
        index: usize,
        left_child_index: usize,
        right_child_index: usize,
    ) -> usize {
        if right_child_index < index
            && matches!(
                self.compare_at(right_child_index, left_child_index),
                ComparatorResult::Less | ComparatorResult::Equal
            )
        {
            right_child_index
        } else {
            left_child_index
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.nodes.swap(i, j);
    }

    fn compare_at(&self, parent_index: usize, child_index: usize) -> ComparatorResult {
        (self.comparator)(&self.nodes[parent_index], &self.nodes[child_index])
    }

    fn should_swap(&self, parent_index: usize, child_index: usize) -> bool {
        if parent_index >= self.size() || child_index >= self.size() {
            return false;
        }
        self.compare_at(parent_index, child_index) == ComparatorResult::Greater
    }

    pub(crate) fn heapify_up(&mut self, start_index: usize) {
        let mut child_index = start_index;
        let mut parent_index = ((child_index as f32 - 1f32) / 2f32).floor() as usize;

        while self.should_swap(parent_index, child_index) {
            self.swap(parent_index, child_index);
            child_index = parent_index;
            parent_index = ((child_index as f32 - 1f32) / 2f32).floor() as usize;
        }
    }

    pub(crate) fn heapify_down(&mut self, start_index: usize) {
        let mut parent_index = start_index;
        while let Some(child_index) = self.pick_child_of(parent_index)
            && self.should_swap(parent_index, child_index)
        {
            self.swap(parent_index, child_index);
            parent_index = child_index;
        }
    }

    pub(crate) fn heapify_down_until(&mut self, index: usize) {
        let mut parent_index = 0;
        let mut left_child_index = 1;
        let mut right_child_index = 2;

        while left_child_index < index {
            let child_index = self.pick_child_before(index, left_child_index, right_child_index);
            if self.should_swap(parent_index, child_index) {
                self.swap(parent_index, child_index);
            }

            parent_index = child_index;
            left_child_index = (parent_index * 2) + 1;
            right_child_index = (parent_index * 2) + 2;
        }
    }

    fn fix(&mut self) {
        // Fix node positions.
        for i in (0..=((self.size() as f32 / 2f32) - 1f32).floor() as i32).rev() {
            self.heapify_down(i as usize);
        }

        // Fix leaf
        for i in (self.size() as f32 / 2f32).floor() as usize..self.size() {
            let value = self.nodes[i];
            if self
                .leaf
                .is_none_or(|leaf| (self.comparator)(&value, &leaf) == ComparatorResult::Greater)
            {
                self.leaf = Some(value);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct Foo {
        id: i32,
    }

    impl Foo {
        fn new(id: i32) -> Self {
            Self { id }
        }
    }

    #[test]
    fn test_min_heap() {
        let comparator = |a: &Foo, b: &Foo| -> ComparatorResult {
            use ComparatorResult::*;
            let r = a.id - b.id;
            if r == 0 {
                Equal
            } else if r > 0 {
                Greater
            } else {
                Less
            }
        };

        let mut heap = Heap::new(comparator, None);
        let mut values = vec![50, 80, 30, 90, 60, 40, 20];

        // Test insert
        for &value in values.iter() {
            heap.insert(Foo::new(value));
        }
        assert_eq!(
            values.len(),
            heap.size(),
            "expected values.len ({}) to == heap.size() ({})",
            values.len(),
            heap.size()
        );

        // Test sort
        let sorted = heap.to_sorted();
        let sorted_vals: Vec<_> = sorted.iter().map(|foo| foo.id).collect();
        assert_eq!(
            sorted,
            Vec::from(heap.nodes.clone()),
            "sorting did not change nodes! expected={sorted:?} | got={:?}",
            heap.nodes
        );
        values.sort();
        values.reverse();
        assert_eq!(
            sorted_vals, values,
            "expected sorted heap to equal sorted values!\n\theap = {sorted_vals:?}\n\tvalues = {values:?}"
        );

        // Test fix after sort
        assert!(!heap.is_valid());
        heap.fix();
        assert!(
            heap.is_valid(),
            "heap is not valid after fix! {:?}",
            heap.nodes
        );
        assert_eq!(
            heap.leaf.expect("to exist"),
            Foo::new(90),
            "expected leaf id to be 90! got {:?}",
            heap.leaf.expect("exist")
        );

        // Test root value
        let root_node = heap.root().expect("exist");
        assert_eq!(root_node, Foo::new(20));

        // Test size
        assert_eq!(heap.size(), values.len());

        // Test is_empty
        assert!(!heap.is_empty());

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
