use crate::heap::{ComparatorResult, Heap};

use std::fmt::Debug;

pub struct PriorityQueue<T, F>
where
    T: Copy + PartialEq + Eq,
    F: Fn(&T, &T) -> ComparatorResult + Clone,
{
    heap: Heap<T, F>,
}

impl<T, F> PriorityQueue<T, F>
where
    T: Copy + PartialEq + Eq + Debug,
    F: Fn(&T, &T) -> ComparatorResult + Clone,
{
    pub fn new(comparator: F, values: Option<Vec<T>>) -> Self {
        Self {
            heap: Heap::new(comparator.clone(), values),
        }
    }

    pub fn front(&self) -> Option<T> {
        self.heap.root()
    }

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

    /// Removes and returns elements that satisfy condition in callback.
    pub fn remove_if<CB>(&mut self, callback: CB) -> Vec<T>
    where
        CB: Fn(&T) -> bool,
    {
        let mut removed = Vec::new();
        let mut dequeued = Vec::new();

        while let Some(popped) = self.pop() {
            if (callback)(&popped) {
                removed.push(popped);
            } else {
                dequeued.push(popped);
            }
        }

        dequeued.iter().for_each(|&t| self.push(t));
        removed
    }

    /// Checks every element in queue against a callback,
    /// if any of the elements meet the provided criterea,
    /// we return true.
    pub fn every<CB>(&mut self, callback: CB) -> bool
    where
        CB: Fn(&T) -> bool,
    {
        let mut dequeued = Vec::new();

        while let Some(popped) = self.pop() {
            dequeued.push(popped);
            if (callback)(&popped) {
                return true;
            }
        }

        dequeued.iter().for_each(|&t| self.push(t));
        false
    }

    pub fn size(&self) -> usize {
        self.heap.size()
    }

    /// Returns sorted array of elements from
    /// highest priority to lowest priority
    pub fn to_vec(&self) -> Vec<T> {
        let mut heap_clone = self.heap.clone();
        heap_clone.sort();
        heap_clone.clone_heap_data()
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

    #[test]
    fn test_priority_queue_with_min_logic() {
        let mut min_queue = PriorityQueue::new(
            |a: &Foo, b: &Foo| -> ComparatorResult {
                if a.id == b.id {
                    ComparatorResult::Equal
                } else if a.id < b.id {
                    ComparatorResult::Less
                } else {
                    ComparatorResult::Greater
                }
            },
            None,
        );

        let values = vec![50, 80, 30, 90, 60, 40, 20];

        // Test adding to queue
        values.iter().for_each(|&v| min_queue.push(Foo::new(v)));
        assert_eq!(values.len(), min_queue.size());

        // Test to_vec
        let mut values_clone_to_vec = values.clone();
        let min_queue_vals_to_vec: Vec<_> = min_queue.to_vec().iter().map(|t| t.id).collect();
        values_clone_to_vec.sort();
        values_clone_to_vec.reverse();
        assert_eq!(values_clone_to_vec, min_queue_vals_to_vec);

        // Test front
        let front = min_queue.front().expect("some");
        assert_eq!(front, Foo::new(20));

        // Test every
        let does_contain = min_queue.every(|e| e.id > 50);
        assert!(does_contain);
    }
}
