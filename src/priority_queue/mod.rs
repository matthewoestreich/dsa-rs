use crate::heap::Heap;

use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display},
};

pub struct PriorityQueue<T, F>
where
    T: PartialEq + Eq + Clone,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    heap: Heap<T, F>,
}

impl<T, F> Debug for PriorityQueue<T, F>
where
    T: PartialEq + Eq + Clone + Debug,
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
    T: PartialEq + Eq + Clone + Display,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PriorityQueue {{ heap: {{ {} }} }}", self.heap)
    }
}

impl<T, F> PartialEq for PriorityQueue<T, F>
where
    T: PartialEq + Eq + Clone,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.heap == other.heap
    }
}

impl<T, F> Eq for PriorityQueue<T, F>
where
    T: PartialEq + Eq + Clone,
    F: Fn(&T, &T) -> Ordering + Copy,
{
}

/// Immutable iteration.
impl<'a, T, F> IntoIterator for &'a PriorityQueue<T, F>
where
    T: PartialEq + Eq + Clone,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.heap).into_iter()
    }
}

/// Consuming iteration.
impl<T, F> IntoIterator for PriorityQueue<T, F>
where
    T: PartialEq + Eq + Clone,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.heap.into_iter()
    }
}

impl<T, F> PriorityQueue<T, F>
where
    T: PartialEq + Eq + Clone,
    F: Fn(&T, &T) -> Ordering + Copy,
{
    pub fn new(compare: F, values: Option<Vec<T>>) -> Self {
        Self {
            heap: Heap::new(compare, values),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.heap.iter()
    }

    /// Returns a reference to the element with highest priority.
    pub fn front(&self) -> Option<&T> {
        self.heap.root()
    }

    /// Returns a reference to the element with lowest priority.
    pub fn back(&self) -> Option<&T> {
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

    /// Extracts, or removes, elements for which `predicate` returns true.
    /// Retains elements for whhich `predicate` returns false.
    /// Returns the elements that were extracted/removed.
    /// The relative order of returned elements is unspecified.
    pub fn extract_if<P>(&mut self, mut predicate: P) -> Vec<T>
    where
        P: FnMut(&T) -> bool,
    {
        let mut extracted = Vec::new();
        let mut retained = Vec::new();

        while let Some(popped) = self.pop() {
            if (predicate)(&popped) {
                extracted.push(popped);
            } else {
                retained.push(popped);
            }
        }

        for t in retained {
            self.push(t);
        }

        extracted
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
        self.heap.to_sorted_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct FooI32 {
        id: i32,
    }

    impl FooI32 {
        fn new(id: i32) -> Self {
            Self { id }
        }
    }

    impl Display for FooI32 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Foo {{ id: {} }}", self.id)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct FooChar {
        char: char,
    }

    impl FooChar {
        fn new(char: char) -> Self {
            Self { char }
        }
    }

    impl Display for FooChar {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Foo {{ char: {} }}", self.char)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        is_active: bool,
    }

    impl User {
        fn new(username: &str, email: &str, sign_in_count: u64, is_active: bool) -> Self {
            Self {
                username: username.into(),
                email: email.into(),
                sign_in_count,
                is_active,
            }
        }
    }

    impl Display for User {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "User {{ username: {}, email: {}, sign_in_count: {}, is_active: {} }}",
                self.username, self.email, self.sign_in_count, self.is_active
            )
        }
    }

    #[test]
    fn test_with_user_struct_max_logic() {
        let max_priority_compare = |a: &User, b: &User| {
            // Users with a greater `sign_in_count` have a higher priority.
            let sign_in_count = a.sign_in_count.cmp(&b.sign_in_count);

            // If two users have the same `sign_in_count`, we use `is_active` field as fallback.
            if sign_in_count == Ordering::Equal {
                // If `a` is `true` and `b` is `false` we want to return `Ordering::Greater`.
                let is_active = a.is_active.cmp(&b.is_active);

                // If two users have the same `sign_in_count` AND `is_active` values,
                // we use `username` field as final fallback.
                if is_active == Ordering::Equal {
                    // Order alphabetically.
                    return b.username.cmp(&a.username);
                }

                // Return `is_active` comparison as first fallback.
                return is_active;
            }

            // Return `sign_in_count` comparison.
            sign_in_count
        };

        let user_values = vec![
            User::new("alice", "alice@example.com", 12, true),
            User::new("bob", "bob@example.com", 3, true),
            User::new("carol", "carol@example.com", 27, false),
            User::new("dave", "dave@example.com", 8, true),
            User::new("eve", "eve@example.com", 41, true),
            User::new("frank", "frank@example.com", 30, false),
            User::new("grace", "grace@example.com", 19, true),
            User::new("heidi", "heidi@example.com", 5, false),
            User::new("ivan", "ivan@example.com", 66, true),
            User::new("judy", "judy@example.com", 2, true),
            User::new("mallory", "mallory@example.com", 91, false),
            User::new("nick", "nick@example.com", 14, true),
            User::new("olivia", "olivia@example.com", 33, true),
            User::new("peggy", "peggy@example.com", 7, false),
            User::new("quentin", "quentin@example.com", 58, true),
            User::new("rachel", "rachel@example.com", 21, true),
            User::new("sybil", "sybil@example.com", 1, true),
            User::new("sybel", "sybel@example.com", 1, true),
            User::new("trent", "trent@example.com", 76, true),
            User::new("ursula", "ursula@example.com", 9, false),
            User::new("victor", "victor@example.com", 100, true),
        ];

        let max_priority_queue = PriorityQueue::new(max_priority_compare, Some(user_values));

        // `front()` returns reference to element with highest priority.
        assert_eq!(
            max_priority_queue.front().expect("some"),
            &User::new("victor", "victor@example.com", 100, true)
        );

        // `back()` returns reference to element with lowest priority.
        assert_eq!(
            max_priority_queue.back().expect("some"),
            &User::new("sybil", "sybil@example.com", 1, true)
        );
    }

    #[test]
    fn test_with_user_struct_min_logic() {
        let compare = |a: &User, b: &User| {
            // Users with a lower `sign_in_count` have a higher priority.
            let sign_in_res = b.sign_in_count.cmp(&a.sign_in_count);
            // If two users have the same `sign_in_count` values,
            // fallback to the `is_active` field.
            if sign_in_res == Ordering::Equal {
                let is_active_res = b.is_active.cmp(&a.is_active);

                // If two users have the same `sign_in_count` AND `is_active`
                // values, use sorting usernames as final fallback.
                if is_active_res == Ordering::Equal {
                    return b.username.cmp(&a.username);
                }

                // Return first `is_active` fallback.
                return is_active_res;
            }

            // Return original `sign_in_count` comparison.
            sign_in_res
        };

        let user_values = vec![
            User::new("alice", "alice@example.com", 12, true),
            User::new("bob", "bob@example.com", 3, true),
            User::new("carol", "carol@example.com", 27, false),
            User::new("dave", "dave@example.com", 8, true),
            User::new("eve", "eve@example.com", 41, true),
            User::new("frank", "frank@example.com", 30, false),
            User::new("grace", "grace@example.com", 19, true),
            User::new("heidi", "heidi@example.com", 5, false),
            User::new("ivan", "ivan@example.com", 66, true),
            User::new("judy", "judy@example.com", 2, true),
            User::new("mallory", "mallory@example.com", 91, false),
            User::new("nick", "nick@example.com", 14, true),
            User::new("olivia", "olivia@example.com", 33, true),
            User::new("peggy", "peggy@example.com", 7, false),
            User::new("quentin", "quentin@example.com", 58, true),
            User::new("rachel", "rachel@example.com", 21, true),
            User::new("sybil", "sybil@example.com", 1, false),
            User::new("sybel", "sybel@example.com", 1, false),
            User::new("trent", "trent@example.com", 76, true),
            User::new("ursula", "ursula@example.com", 9, false),
            User::new("victor", "victor@example.com", 100, true),
        ];

        let min_priority_queue = PriorityQueue::new(compare, Some(user_values));

        // `front()` returns reference to element with highest priority.
        assert_eq!(
            min_priority_queue.front().expect("some"),
            &User::new("sybel", "sybel@example.com", 1, false)
        );

        // `back()` returns reference to element with lowest priority.
        assert_eq!(
            min_priority_queue.back().expect("some"),
            &User::new("victor", "victor@example.com", 100, true)
        );
    }

    #[test]
    fn test_priority_queue_with_min_logic() {
        let compare = |a: &FooI32, b: &FooI32| b.id.cmp(&a.id);
        let mut min_queue = PriorityQueue::new(compare, None);

        let values = vec![50, 80, 30, 90, 60, 40, 20];

        // Test adding to queue
        values.iter().for_each(|&v| min_queue.push(FooI32::new(v)));
        assert_eq!(values.len(), min_queue.size());

        println!("{min_queue}");

        // Test to_vec
        let mut values_clone_to_vec = values.clone();
        values_clone_to_vec.sort();
        let min_queue_vals_to_vec: Vec<_> =
            min_queue.to_sorted_vec().iter().map(|t| t.id).collect();
        assert_eq!(values_clone_to_vec, min_queue_vals_to_vec);

        // Test front
        let front = min_queue.front().expect("some");
        assert_eq!(front, &FooI32::new(20));

        // Test every
        let does_contain = min_queue.any(|e| e.id > 50);
        assert!(does_contain);

        let removed = min_queue.extract_if(|e| e.id > 20);
        assert_eq!(removed.len(), values.len() - 1);
        assert_eq!(min_queue.to_sorted_vec(), vec![FooI32::new(20)]);
        assert_eq!(min_queue.size(), 1);
    }

    #[test]
    fn test_priority_queue_with_max_logic() {
        let compare = |a: &FooChar, b: &FooChar| a.char.cmp(&b.char);
        let mut max_queue = PriorityQueue::new(compare, None);

        let values = vec!['m', 'x', 'f', 'b', 'z', 'k', 'c'];

        // Test adding to queue
        values.iter().for_each(|&v| max_queue.push(FooChar::new(v)));
        assert_eq!(values.len(), max_queue.size());

        println!("{max_queue}");

        // Test to_vec
        let mut values_clone_to_vec = values.clone();
        values_clone_to_vec.sort_by(|a, b| b.cmp(a));
        let min_queue_vals_to_vec: Vec<_> =
            max_queue.to_sorted_vec().iter().map(|t| t.char).collect();
        assert_eq!(values_clone_to_vec, min_queue_vals_to_vec);

        // Test front
        let front = max_queue.front().expect("some");
        assert_eq!(front, &FooChar::new('z'));

        // Test every
        let does_contain = max_queue.any(|e| e.char > 'a');
        assert!(does_contain);

        let removed = max_queue.extract_if(|e| e.char < 'z');
        assert_eq!(removed.len(), values.len() - 1);
        assert_eq!(max_queue.to_sorted_vec(), vec![FooChar::new('z')]);
        assert_eq!(max_queue.size(), 1);
    }
}
