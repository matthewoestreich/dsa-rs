use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new(head: T) -> Self {
        Self {
            head: Some(Node::new(head)),
            len: 1,
        }
    }

    /// Returns reference to head.
    pub fn head(&self) -> Option<&T> {
        self.head.as_deref().map(|h| &h.value)
    }

    /// Returns mutable reference to head.
    pub fn head_mut(&mut self) -> Option<&mut T> {
        self.head.as_deref_mut().map(|h| &mut h.value)
    }

    /// Returns reference to tail.
    pub fn tail(&self) -> Option<&T> {
        let mut curr = &self.head;

        while let Some(curr_node) = curr {
            if curr_node.next.is_none() {
                return Some(&curr_node.value);
            }
            curr = &curr_node.next;
        }

        None
    }

    /// Returns mutable reference to tail.
    pub fn tail_mut(&mut self) -> Option<&mut T> {
        let mut curr = &mut self.head;

        while let Some(curr_node) = curr {
            if curr_node.next.is_none() {
                return Some(&mut curr_node.value);
            }
            curr = &mut curr_node.next;
        }

        None
    }

    /// Removes and returns head. Assigns `popped_head.next` as new head.
    pub fn pop_head(&mut self) -> Option<T> {
        self.remove(0)
    }

    /// Removes and returns tail. Assigns node that came before popped tail as new tail.
    pub fn pop_tail(&mut self) -> Option<T> {
        self.remove(self.len - 1)
    }

    /// Inserts new value into linked list at the front (becomes head).
    pub fn insert_front(&mut self, value: T) {
        let new_head = Node::new_with_next(value, self.head.take());
        self.head = Some(new_head);
        self.len += 1;
    }

    /// Inserts new value into linked list at the end (becomes tail).
    pub fn insert_back(&mut self, value: T) {
        let mut curr = &mut self.head;

        while let Some(curr_node) = curr {
            if curr_node.next.is_none() {
                curr_node.next = Some(Node::new(value));
                self.len += 1;
                return;
            }
            curr = &mut curr_node.next;
        }
    }

    pub fn iter(&self) -> SinglyLinkedListIter<'_, T> {
        SinglyLinkedListIter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> SinglyLinkedListIterMut<'_, T> {
        SinglyLinkedListIterMut {
            next: self.head.as_deref_mut(),
        }
    }

    /// Indexing is zero based.
    /// If you have 3 elements in your list, the third element will be at index 2.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        let mut i = 0;
        let mut curr = &mut self.head;

        while i < self.len && curr.is_some() {
            if i == index {
                let rest = curr.take().expect("already checked is_some");
                *curr = rest.next;
                self.len -= 1;
                return Some(rest.value);
            }
            i += 1;
            curr = &mut curr.as_mut().expect("already checked is_some").next;
        }

        None
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Debug for SinglyLinkedList<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkedList")
            .field("head", &self.head)
            .field("size", &self.len)
            .finish()
    }
}

impl<T> Display for SinglyLinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkedList {{ ")?;
        let mut next = self.head.as_ref();
        while let Some(next_node) = next {
            let curr = next_node;
            if curr.next.is_some() {
                write!(f, "{} -> ", curr.value)?;
            } else {
                write!(f, "{}", curr.value)?;
            }
            next = curr.next.as_ref();
        }
        write!(f, " }}")
    }
}

impl<T> PartialEq for SinglyLinkedList<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        let mut a = &self.head;
        let mut b = &other.head;

        while let Some(node_a) = a.as_ref()
            && let Some(node_b) = b.as_ref()
            && node_a.value == node_b.value
        {
            a = &node_a.next;
            b = &node_b.next;
        }

        b.is_none() && a.is_none()
    }
}

impl<T> Eq for SinglyLinkedList<T> where T: Eq {}

impl<T> TryFrom<&[T]> for SinglyLinkedList<T>
where
    T: Clone,
{
    type Error = SinglyLinkedListError;

    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        if slice.is_empty() {
            return Err(SinglyLinkedListError::EmptySource);
        }

        let mut this = Self::new(slice[0].clone());
        for v in &slice[1..] {
            this.insert_back(v.clone());
        }

        Ok(this)
    }
}

/// Consumes the array!
impl<T, const N: usize> TryFrom<[T; N]> for SinglyLinkedList<T> {
    type Error = SinglyLinkedListError;

    fn try_from(array: [T; N]) -> Result<Self, Self::Error> {
        if N == 0 {
            return Err(SinglyLinkedListError::EmptySource);
        }

        let mut it = array.into_iter();
        let first = it.next().expect("verified not empty");

        let mut this = Self::new(first);
        for item in it {
            this.insert_back(item);
        }

        Ok(this)
    }
}

impl<T, const N: usize> TryFrom<&[T; N]> for SinglyLinkedList<T>
where
    T: Clone,
{
    type Error = SinglyLinkedListError;

    fn try_from(array: &[T; N]) -> Result<Self, Self::Error> {
        if array.is_empty() {
            return Err(SinglyLinkedListError::EmptySource);
        }

        let mut this = Self::new(array[0].clone());
        for v in &array[1..] {
            this.insert_back(v.clone());
        }

        Ok(this)
    }
}

impl<T> TryFrom<Vec<T>> for SinglyLinkedList<T>
where
    T: Clone,
{
    type Error = SinglyLinkedListError;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        Self::try_from(vec.as_slice())
    }
}

/// Consuming iteration.
impl<T> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = SinglyLinkedListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        SinglyLinkedListIntoIter { list: self }
    }
}

/// Immutable borrowing iteration.
impl<'a, T> IntoIterator for &'a SinglyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = SinglyLinkedListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Mutable borrowing iteration.
impl<'a, T> IntoIterator for &'a mut SinglyLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = SinglyLinkedListIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/* ============================================================================================ */
/* ==================================== Iterators ============================================= */
/* ============================================================================================ */

pub struct SinglyLinkedListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

/// Iterator
impl<'a, T> Iterator for SinglyLinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

pub struct SinglyLinkedListIterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for SinglyLinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

pub struct SinglyLinkedListIntoIter<T> {
    list: SinglyLinkedList<T>,
}

/// Consuming iterator
impl<T> Iterator for SinglyLinkedListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_head()
    }
}

/* ============================================================================================ */
/* ==================================== Node ================================================== */
/* ============================================================================================ */

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Box<Self> {
        Box::new(Self { value, next: None })
    }

    fn new_with_next(value: T, next: Option<Box<Node<T>>>) -> Box<Self> {
        Box::new(Self { value, next })
    }
}

impl<T> Debug for Node<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("value", &self.value)
            .field("next", &self.next)
            .finish()
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ value: {},", self.value)?;
        if let Some(n) = &self.next {
            write!(f, " next: {}", n)?;
        } else {
            write!(f, " next: None")?;
        }
        write!(f, " }}")
    }
}

/* ============================================================================================ */
/* ==================================== SinglyLinkedListError ================================= */
/* ============================================================================================ */

#[derive(Debug)]
pub enum SinglyLinkedListError {
    EmptySource,
}

impl Display for SinglyLinkedListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SinglyLinkedListError::EmptySource => write!(
                f,
                "SinglyLinkedListError::EmptySource(source must contain at least one element)"
            ),
        }
    }
}

impl Error for SinglyLinkedListError {}

/* ============================================================================================ */
/* ==================================== TESTS ================================================= */
/* ============================================================================================ */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_front() {
        let head = 0;
        let mut list = SinglyLinkedList::new(head);
        let new_head = 1;
        list.insert_front(new_head);
        list.insert_front(2);
        assert_eq!(list.len(), 3);
        assert_eq!(list.head(), Some(&2));
    }

    #[test]
    fn test_insert_back() {
        let mut list = SinglyLinkedList::new(0);
        list.insert_front(1);
        list.insert_front(2);
        list.insert_back(99);
        assert_eq!(list.len(), 4);
        assert_eq!(list.tail(), Some(&99));
    }

    #[test]
    fn test_is_empty() {
        let mut list_a = SinglyLinkedList::new(0);
        _ = list_a.remove(0);
        assert!(list_a.is_empty());
        assert_eq!(list_a.len(), 0);

        let mut list_b = SinglyLinkedList::new(0);
        _ = list_b.pop_head();
        assert!(list_b.is_empty());
        assert_eq!(list_b.len(), 0);
    }

    #[test]
    fn test_head_mut() {
        let mut list = SinglyLinkedList::new(0);
        list.insert_back(1);
        if let Some(head_mut) = list.head_mut() {
            *head_mut = 99;
        }
        assert_eq!(list.head(), Some(&99));
    }

    #[test]
    fn test_tail_mut() {
        let mut list = SinglyLinkedList::new(0);
        list.insert_back(1);
        if let Some(tail_mut) = list.tail_mut() {
            *tail_mut = 99;
        }
        assert_eq!(list.tail(), Some(&99));
        println!("{list}");
    }

    #[test]
    fn test_iter() {
        let mut list = SinglyLinkedList::new(0);
        for v in [1, 2, 3, 4, 5] {
            list.insert_back(v);
        }

        let mut list_vec = vec![];
        let list_expected_vec = vec![0, 1, 2, 3, 4, 5];
        for &v in list.iter() {
            list_vec.push(v);
        }

        assert_eq!(list_vec, list_expected_vec);
    }

    #[test]
    fn test_iter_mut() {
        let multiplier = 10;
        let values = [0, 1, 2, 3, 4, 5];
        let expected_vec = values.map(|v| v * multiplier);

        let mut expected_list = SinglyLinkedList::new(expected_vec[0]);
        for &v in &expected_vec[1..] {
            expected_list.insert_back(v);
        }

        let mut list = SinglyLinkedList::new(values[0]);
        for &v in &values[1..] {
            list.insert_back(v);
        }

        let mut list_vec = vec![];
        for v in &mut list {
            *v *= multiplier;
            list_vec.push(*v);
        }

        assert_eq!(list_vec, expected_vec);
        assert_eq!(list, expected_list);
    }

    #[test]
    fn test_remove_at() {
        let mut expected_list = SinglyLinkedList::new("a");
        for v in ["b", "c"] {
            expected_list.insert_back(v);
        }

        let mut list = SinglyLinkedList::new("a");
        for v in ["b", "c", "d"] {
            list.insert_back(v);
        }

        // Test out of bounds `remove`
        assert_eq!(list.remove(1000000), None);
        assert_eq!(list.len(), 4);
        // Test zero-based indexing
        assert_eq!(list.remove(4), None);
        // Test removing by valid index
        assert_eq!(list.remove(3), Some("d"));
        assert_eq!(list.len(), 3);
        // Ensure our list is what we expect
        assert_eq!(list, expected_list);
    }

    #[test]
    fn test_pop_head() {
        let mut list = SinglyLinkedList::new(33);
        for v in [34, 67, 22, 90, 81] {
            list.insert_back(v);
        }
        assert_eq!(list.pop_head(), Some(33));
        // Test if head was updated.
        assert_eq!(list.head(), Some(&34));
        assert_eq!(list.tail(), Some(&81));
    }

    #[test]
    fn test_pop_tail() {
        let mut list = SinglyLinkedList::new(33);
        for v in [34, 67, 22, 90, 81] {
            list.insert_back(v);
        }
        assert_eq!(list.pop_tail(), Some(81));
        // Test if tail was updated.
        assert_eq!(list.tail(), Some(&90));
        assert_eq!(list.head(), Some(&33));
    }

    #[test]
    fn test_equality() {
        let mut list_a = SinglyLinkedList::new("a");
        list_a.insert_back("b");
        let list_b = SinglyLinkedList::new("a");
        assert_ne!(list_a, list_b);

        let data = ["z", "x", "q", "w"];
        let mut list_c = SinglyLinkedList::new(data[0]);
        let mut list_d = SinglyLinkedList::new(data[0]);
        for v in &data[1..] {
            list_c.insert_back(v);
            list_d.insert_back(v);
        }
        assert_eq!(list_c, list_d);

        let mut list_e = SinglyLinkedList::new(0);
        list_e.insert_back(1);
        list_e.insert_back(2);
        let mut list_f = SinglyLinkedList::new(0);
        list_f.insert_back(1);
        list_f.insert_back(2);
        list_f.insert_back(3);
        assert_ne!(list_e, list_f);

        let mut list_g = SinglyLinkedList::new(0);
        list_g.insert_back(1);
        list_g.insert_back(2);
        list_g.insert_back(3);
        let mut list_h = SinglyLinkedList::new(0);
        list_h.insert_back(1);
        list_h.insert_back(2);
        list_h.insert_back(99);
        assert_ne!(list_g, list_h);
    }

    #[test]
    fn test_try_from() {
        let vec_data = Vec::from(["a", "b", "c", "d", "e", "f"]);
        let vec_data_len = vec_data.len();
        let vec_list = SinglyLinkedList::try_from(vec_data).expect("no errors");
        assert_eq!(vec_list.head(), Some(&"a"));
        assert_eq!(vec_list.tail(), Some(&"f"));
        assert_eq!(vec_list.len(), vec_data_len);

        let arr_data = ["aa", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"];
        let arr_data_len = arr_data.len();
        let arr_list = SinglyLinkedList::try_from(arr_data).expect("no errors");
        assert_eq!(arr_list.head(), Some(&"aa"));
        assert_eq!(arr_list.tail(), Some(&"k"));
        assert_eq!(arr_list.len(), arr_data_len);

        let arr_ref_data = ["aaa", "b", "c", "d"];
        let arr_ref_data_len = arr_ref_data.len();
        let arr_ref_list = SinglyLinkedList::try_from(&arr_ref_data).expect("no errors");
        assert_eq!(arr_ref_list.head(), Some(&"aaa"));
        assert_eq!(arr_ref_list.tail(), Some(&"d"));
        assert_eq!(arr_ref_list.len(), arr_ref_data_len);

        let slice_data = ["aaaa", "b", "c", "d", "e"];
        let slice_data_len = slice_data.len();
        let slice_list = SinglyLinkedList::try_from(&slice_data[..]).expect("no errors");
        assert_eq!(slice_list.head(), Some(&"aaaa"));
        assert_eq!(slice_list.tail(), Some(&"e"));
        assert_eq!(slice_list.len(), slice_data_len);
    }
}
