use crate::heap::Heap;

use std::{
    cmp::Ordering,
    collections::vec_deque,
    fmt::{self, Debug, Display},
};

pub struct PriorityQueue<T, F>
where
    T: Copy + PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    heap: Heap<T, F>,
}

impl<T, F> Debug for PriorityQueue<T, F>
where
    T: Copy + PartialEq + Eq + Debug,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PriorityQueue")
            .field("heap", &self.heap)
            .finish()
    }
}

impl<T, F> Display for PriorityQueue<T, F>
where
    T: Copy + PartialEq + Eq + Display,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PriorityQueue {{ heap: {{ {} }} }}", self.heap)
    }
}

impl<T, F> PartialEq for PriorityQueue<T, F>
where
    T: Copy + PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.heap == other.heap
    }
}

impl<T, F> Eq for PriorityQueue<T, F>
where
    T: Copy + PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
}

/// Immutable iteration.
impl<'a, T, F> IntoIterator for &'a PriorityQueue<T, F>
where
    T: Copy + PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    type Item = &'a T;
    type IntoIter = vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.heap).into_iter()
    }
}

/// Consuming iteration.
impl<T, F> IntoIterator for PriorityQueue<T, F>
where
    T: Copy + PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    type Item = T;
    type IntoIter = vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.heap.into_iter()
    }
}

impl<T, F> PriorityQueue<T, F>
where
    T: Copy + PartialEq + Eq,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    pub fn new(comparator: F, values: Option<Vec<T>>) -> Self {
        Self {
            heap: Heap::new(comparator, values),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.heap.iter()
    }

    /// Returns a copy of the element with highest priority.
    pub fn front(&self) -> Option<T> {
        self.heap.root()
    }

    /// Returns a copy of the element with lowest priority.
    pub fn back(&self) -> Option<T> {
        self.heap.leaf()
    }

    pub fn enqueue(&mut self, value: T) {
        self.heap.insert(value);
    }

    /// Alias for enqueue.
    pub fn push(&mut self, value: T) {
        self.enqueue(value);
    }

    /// Removes and returns the element with the highest priority.
    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.extract_root()
    }

    /// Alias for `dequeue`.
    pub fn pop(&mut self) -> Option<T> {
        self.dequeue()
    }

    /// Retains elements for which `predicate` returns true.
    /// Returns the elements that were removed.
    /// The relative order of returned elements is unspecified.
    pub fn drain_filter<P>(&mut self, mut predicate: P) -> Vec<T>
    where
        P: FnMut(&T) -> bool,
    {
        let mut removed = Vec::new();
        let mut retained = Vec::new();

        while let Some(popped) = self.pop() {
            if (predicate)(&popped) {
                retained.push(popped);
            } else {
                removed.push(popped);
            }
        }

        for t in retained {
            self.push(t);
        }

        removed
    }

    /// Shorthand for `self.iter().any(...)`
    pub fn any<P>(&self, predicate: P) -> bool
    where
        P: FnMut(&T) -> bool,
    {
        self.heap.iter().any(predicate)
    }

    pub fn size(&self) -> usize {
        self.heap.size()
    }

    /// Returns a copy of underlying heap as sorted `Vec` of elements.
    /// Elements are sorted from highest priority to lowest priority.
    pub fn to_sorted_vec(&self) -> Vec<T> {
        let mut heap_clone = self.heap.clone();
        heap_clone.sort();
        heap_clone.take_heap_data()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

#[cfg(test)]
mod test {
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
    fn test_priority_queue_with_min_logic() {
        let comparator = |a: &Foo, b: &Foo| a.id.cmp(&b.id);
        let mut min_queue = PriorityQueue::new(comparator, None);

        let values = vec![50, 80, 30, 90, 60, 40, 20];

        // Test adding to queue
        values.iter().for_each(|&v| min_queue.push(Foo::new(v)));
        assert_eq!(values.len(), min_queue.size());

        println!("{min_queue}");

        // Test to_vec
        let mut values_clone_to_vec = values.clone();
        let min_queue_vals_to_vec: Vec<_> =
            min_queue.to_sorted_vec().iter().map(|t| t.id).collect();
        values_clone_to_vec.sort_by(|a, b| b.cmp(a));
        assert_eq!(values_clone_to_vec, min_queue_vals_to_vec);

        // Test front
        let front = min_queue.front().expect("some");
        assert_eq!(front, Foo::new(20));

        // Test every
        let does_contain = min_queue.any(|e| e.id > 50);
        assert!(does_contain);

        let removed = min_queue.drain_filter(|e| e.id == 20);
        assert_eq!(removed.len(), values.len() - 1);
        assert_eq!(min_queue.to_sorted_vec(), vec![Foo::new(20)]);
        assert_eq!(min_queue.size(), 1);
    }
}
