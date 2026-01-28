use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new(head: T) -> Self {
        Self {
            head: Some(Node::new(head)),
            size: 1,
        }
    }

    pub fn head(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(h) => Some(&h.value),
            None => None,
        }
    }

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

    pub fn pop_head(&mut self) -> Option<T> {
        self.remove_at(0)
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        self.remove_at(self.size - 1)
    }

    pub fn insert_first(&mut self, value: T) {
        let new_head = Node::new_with_next(value, self.head.take());
        self.head = Some(new_head);
        self.size += 1;
    }

    pub fn insert_last(&mut self, value: T) {
        let mut curr = &mut self.head;

        while let Some(curr_node) = curr {
            if curr_node.next.is_none() {
                curr_node.next = Some(Node::new(value));
                self.size += 1;
                return;
            }
            curr = &mut curr_node.next;
        }
    }

    /// Indexing is zero based.
    /// If you have 3 elements in your list, the third element will be at index 2.
    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let mut i = 0;
        let mut curr = &mut self.head;

        while i < self.size && curr.is_some() {
            if i == index {
                let rest = curr.take().expect("already checked is_some");
                *curr = rest.next;
                self.size -= 1;
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

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T> Debug for SinglyLinkedList<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkedList")
            .field("head", &self.head)
            .field("size", &self.size)
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
        if self.size != other.size {
            return false;
        }

        let mut a = &self.head;
        let mut b = &other.head;

        while let Some(a_node) = a {
            if b.is_none() {
                return false;
            }

            let b_node = b.as_ref().expect("some");
            if a_node.value != b_node.value {
                return false;
            }

            a = &a_node.next;
            b = &b_node.next;
        }

        b.is_none()
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
            this.insert_last(v.clone());
        }
        Ok(this)
    }
}

/// Consumes the slice!
impl<T, const N: usize> TryFrom<[T; N]> for SinglyLinkedList<T> {
    type Error = SinglyLinkedListError;

    fn try_from(slice: [T; N]) -> Result<Self, Self::Error> {
        if N == 0 {
            return Err(SinglyLinkedListError::EmptySource);
        }

        let mut it = slice.into_iter();
        let first = it.next().expect("verified not empty");

        let mut this = Self::new(first);
        for item in it {
            this.insert_last(item);
        }

        Ok(this)
    }
}

impl<T, const N: usize> TryFrom<&[T; N]> for SinglyLinkedList<T>
where
    T: Clone,
{
    type Error = SinglyLinkedListError;

    fn try_from(slice: &[T; N]) -> Result<Self, Self::Error> {
        if slice.is_empty() {
            return Err(SinglyLinkedListError::EmptySource);
        }

        let mut this = Self::new(slice[0].clone());
        for v in &slice[1..] {
            this.insert_last(v.clone());
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
                "SinglyLinkedListError::EmptySource -> Cannot convert empty slice into LinkedList"
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
    fn test_insert_first() {
        let head = 0;
        let mut list = SinglyLinkedList::new(head);
        let new_head = 1;
        list.insert_first(new_head);
        list.insert_first(2);
        assert_eq!(3, list.size());
        assert_eq!(Some(&2), list.head());
    }

    #[test]
    fn test_insert_last() {
        let mut list = SinglyLinkedList::new(0);
        list.insert_first(1);
        list.insert_first(2);
        list.insert_last(99);
        assert_eq!(4, list.size());
        assert_eq!(Some(&99), list.tail());
    }

    #[test]
    fn test_remove_at() {
        let mut expected_list = SinglyLinkedList::new("a");
        for v in ["b", "c"] {
            expected_list.insert_last(v);
        }

        let mut list = SinglyLinkedList::new("a");
        for v in ["b", "c", "d"] {
            list.insert_last(v);
        }

        // Test out of bounds `remove_at`
        assert_eq!(None, list.remove_at(1000000));
        assert_eq!(4, list.size());
        // Test zero-based indexing
        assert_eq!(None, list.remove_at(4));
        // Test removing by valid index
        assert_eq!(Some("d"), list.remove_at(3));
        assert_eq!(3, list.size());
        // Ensure our list is what we expect
        assert_eq!(list, expected_list);
    }

    #[test]
    fn test_pop_head() {
        let mut list = SinglyLinkedList::new(33);
        for v in [34, 67, 22, 90, 81] {
            list.insert_last(v);
        }
        let head = list.pop_head();
        assert_eq!(Some(33), head);
        // Test if head was updated.
        assert_eq!(Some(&34), list.head());
        assert_eq!(Some(&81), list.tail());
    }

    #[test]
    fn test_pop_tail() {
        let mut list = SinglyLinkedList::new(33);
        for v in [34, 67, 22, 90, 81] {
            list.insert_last(v);
        }
        let tail = list.pop_tail();
        assert_eq!(Some(81), tail);
        // Test if tail was updated.
        assert_eq!(Some(&90), list.tail());
        assert_eq!(Some(&33), list.head());
    }

    #[test]
    fn test_equality() {
        let mut list_a = SinglyLinkedList::new("a");
        list_a.insert_last("b");
        let list_b = SinglyLinkedList::new("a");
        assert_ne!(list_a, list_b);

        let data = ["z", "x", "q", "w"];
        let mut list_c = SinglyLinkedList::new(data[0]);
        let mut list_d = SinglyLinkedList::new(data[0]);
        for v in &data[1..] {
            list_c.insert_last(v);
            list_d.insert_last(v);
        }
        assert_eq!(list_c, list_d);

        // Due to how our equality check is written, we
        // should test for the case where the "left hand side" (a)
        // has no elements left but the "right hand side" (b) does.
        let mut list_e = SinglyLinkedList::new(0);
        list_e.insert_last(1);
        list_e.insert_last(2);
        let mut list_f = SinglyLinkedList::new(0);
        list_f.insert_last(1);
        list_f.insert_last(2);
        list_f.insert_last(3);
        assert_ne!(list_e, list_f);
    }

    #[test]
    fn test_try_from() {
        let vec_data = Vec::from(["a", "b", "c", "d", "e", "f"]);
        let vec_list = SinglyLinkedList::try_from(vec_data).expect("no errors");
        assert_eq!(Some(&"a"), vec_list.head());
        assert_eq!(Some(&"f"), vec_list.tail());

        let arr_data = ["a", "b", "c", "d", "e", "f"];
        let arr_list = SinglyLinkedList::try_from(arr_data).expect("no errors");
        assert_eq!(Some(&"a"), arr_list.head());
        assert_eq!(Some(&"f"), arr_list.tail());

        let arr_ref_data = ["a", "b", "c", "d", "e", "f"];
        let arr_ref_list = SinglyLinkedList::try_from(&arr_ref_data).expect("no errors");
        assert_eq!(Some(&"a"), arr_ref_list.head());
        assert_eq!(Some(&"f"), arr_ref_list.tail());

        let slice_data = ["a", "b", "c", "d", "e", "f"];
        let slice_list = SinglyLinkedList::try_from(&slice_data[..]).expect("no errors");
        assert_eq!(Some(&"a"), slice_list.head());
        assert_eq!(Some(&"f"), slice_list.tail());
    }
}
