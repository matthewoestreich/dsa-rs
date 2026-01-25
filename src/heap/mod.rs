#[derive(Debug, PartialEq, Eq)]
pub enum ComparatorResult {
    Greater,
    Less,
    Equal,
}

pub struct Heap<T, F>
where
    T: Copy,
    F: Fn(&T, &T) -> ComparatorResult,
{
    comparator: F,
    pub(crate) nodes: Vec<T>,
    pub(crate) leaf: Option<T>,
}

#[allow(dead_code)]
impl<T, F> Heap<T, F>
where
    T: Copy,
    F: Fn(&T, &T) -> ComparatorResult,
{
    pub fn new(comparator: F, values: Option<Vec<T>>) -> Self {
        let mut this = Self {
            comparator,
            nodes: values.unwrap_or_default(),
            leaf: None,
        };
        if !this.nodes.is_empty() {
            this.fix();
        }
        this
    }

    pub fn insert(&mut self, value: T) {
        self.nodes.push(value);
        self.heapify_up(self.nodes.len() - 1);
        if self
            .leaf
            .is_none_or(|leaf| (self.comparator)(&value, &leaf) == ComparatorResult::Greater)
        {
            self.leaf = Some(value);
        }
    }

    /// Sorts heap data in place.
    pub fn sort(&mut self) {
        for i in (0..self.nodes.len()).rev() {
            self.swap(0, i);
            self.heapify_down_until(i);
        }
    }

    /// Sorts in place as well as returns copy of sorted data.
    pub fn to_sorted(&mut self) -> Vec<T> {
        self.sort();
        self.nodes.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid_from_index(0)
    }

    fn is_valid_from_index(&self, parent_index: usize) -> bool {
        let mut is_valid_left = true;
        let mut is_valid_right = true;

        if self.has_left_child(parent_index) {
            let left_child_index = (parent_index * 2) + 1;
            if self.compare_at(parent_index, left_child_index) == ComparatorResult::Greater {
                return false;
            }
            is_valid_left = self.is_valid_from_index(left_child_index);
        }

        if self.has_right_child(parent_index) {
            let right_child_index = (parent_index * 2) + 2;
            if self.compare_at(parent_index, right_child_index) == ComparatorResult::Greater {
                return false;
            }
            is_valid_right = self.is_valid_from_index(right_child_index);
        }

        is_valid_left && is_valid_right
    }

    fn has_left_child(&self, parent_index: usize) -> bool {
        let left_child_index = (parent_index * 2) + 1;
        left_child_index < self.nodes.len()
    }

    fn has_right_child(&self, parent_index: usize) -> bool {
        let right_child_index = (parent_index * 2) + 2;
        right_child_index < self.nodes.len()
    }

    fn pick_child_of(&self, parent_index: usize) -> Option<usize> {
        if !self.has_left_child(parent_index) && !self.has_right_child(parent_index) {
            return None;
        }

        let left_child_index = (parent_index * 2) + 1;
        let right_child_index = (parent_index * 2) + 2;

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
        if parent_index >= self.nodes.len() || child_index >= self.nodes.len() {
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
        for i in (0..=((self.nodes.len() as f32 / 2f32) - 1f32).floor() as i32).rev() {
            self.heapify_down(i as usize);
        }

        // Fix leaf
        for i in (self.nodes.len() as f32 / 2f32).floor() as usize..self.nodes.len() {
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
            heap.nodes.len(),
            "expected values.len ({}) to == heap.nodes.len ({})",
            values.len(),
            heap.nodes.len()
        );

        // Test sort
        let sorted = heap.to_sorted();
        let sorted_vals: Vec<_> = sorted.iter().map(|foo| foo.id).collect();
        assert_eq!(
            sorted, heap.nodes,
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
    }
}
