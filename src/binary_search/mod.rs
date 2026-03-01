use std::cmp;

pub enum Ordering {
    Ascending,
    Descending,
}

/// Assumes `elements` are already sorted!!!
/// Returns the index of target, if found.
pub fn find<T>(elements: &[T], elements_order: Ordering, target: &T) -> Option<usize>
where
    T: PartialEq + Eq + PartialOrd + Ord,
{
    let mut left = 0;
    let mut right = elements.len();

    while left < right {
        let mid = left + (right - left) / 2;
        let candidate = &elements[mid];

        let ordering = match elements_order {
            Ordering::Ascending => candidate.cmp(target),
            Ordering::Descending => candidate.cmp(target).reverse(),
        };

        match ordering {
            cmp::Ordering::Less => left = mid + 1,
            cmp::Ordering::Greater => right = mid,
            cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}

// *************** BinarySearch trait ************************************************

pub trait BinarySearch<T>
where
    Self: AsRef<[T]>,
    T: Ord,
{
    fn find_binary_search(&self, arr_order: Ordering, target: &T) -> Option<usize> {
        find(self.as_ref(), arr_order, target)
    }
}

impl<T, C> BinarySearch<T> for C
where
    T: Ord,
    C: AsRef<[T]>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incorrect_ordering_asc() {
        // vec is in descending order but we provide BinarySearchOrdering::Ascending
        let v = vec![5, 4, 3, 2, 1];
        assert_eq!(find(&v, Ordering::Ascending, &2), None);
    }

    #[test]
    fn incorrect_ordering_desc() {
        // vec is in ascending order but we provide BinarySearchOrdering::Descending
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(find(&v, Ordering::Descending, &2), None);
    }

    #[test]
    fn ascending_finds_middle() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(find(&v, Ordering::Ascending, &3), Some(2));
    }

    #[test]
    fn ascending_finds_first() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(find(&v, Ordering::Ascending, &1), Some(0));
    }

    #[test]
    fn ascending_finds_last() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(find(&v, Ordering::Ascending, &5), Some(4));
    }

    #[test]
    fn ascending_not_found() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(find(&v, Ordering::Ascending, &10), None);
    }

    #[test]
    fn descending_finds_middle() {
        let v = vec![5, 4, 3, 2, 1];
        assert_eq!(find(&v, Ordering::Descending, &3), Some(2));
    }

    #[test]
    fn descending_finds_first() {
        let v = vec![5, 4, 3, 2, 1];
        assert_eq!(find(&v, Ordering::Descending, &5), Some(0));
    }

    #[test]
    fn descending_finds_last() {
        let v = vec![5, 4, 3, 2, 1];
        assert_eq!(find(&v, Ordering::Descending, &1), Some(4));
    }

    #[test]
    fn descending_not_found() {
        let v = vec![5, 4, 3, 2, 1];
        assert_eq!(find(&v, Ordering::Descending, &10), None);
    }

    #[test]
    fn empty_slice() {
        let v: Vec<i32> = vec![];
        assert_eq!(find(&v, Ordering::Ascending, &1), None);
    }

    #[test]
    fn single_element_found() {
        let v = vec![42];
        assert_eq!(find(&v, Ordering::Ascending, &42), Some(0));
    }

    #[test]
    fn single_element_not_found() {
        let v = vec![42];
        assert_eq!(find(&v, Ordering::Ascending, &1), None);
    }

    #[test]
    fn duplicates_returns_some_valid_index() {
        let v = vec![1, 2, 2, 2, 3];
        let result = find(&v, Ordering::Ascending, &2);

        assert!(result.is_some());
        let idx = result.unwrap();
        assert_eq!(v[idx], 2);
    }

    #[test]
    fn works_with_strings() {
        let v = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
        ];

        assert_eq!(
            find(&v, Ordering::Ascending, &String::from("banana")),
            Some(1)
        );
    }

    #[test]
    fn binary_search_trait() {
        use Ordering as Ord;

        let arr_asc = [1, 3, 5, 7, 9];
        let arr_desc = [9, 7, 5, 3, 1];

        // Helper to test a container that implements BinarySearch
        fn test_container<C>(ctr: C, order: Ordering)
        where
            C: BinarySearch<i32>,
        {
            match order {
                Ord::Ascending => {
                    assert_eq!(ctr.find_binary_search(Ord::Ascending, &1), Some(0));
                    assert_eq!(ctr.find_binary_search(Ord::Ascending, &5), Some(2));
                    assert_eq!(ctr.find_binary_search(Ord::Ascending, &9), Some(4));
                    assert_eq!(ctr.find_binary_search(Ord::Ascending, &10), None);
                }
                Ord::Descending => {
                    assert_eq!(ctr.find_binary_search(Ord::Descending, &9), Some(0));
                    assert_eq!(ctr.find_binary_search(Ord::Descending, &5), Some(2));
                    assert_eq!(ctr.find_binary_search(Ord::Descending, &1), Some(4));
                    assert_eq!(ctr.find_binary_search(Ord::Descending, &0), None);
                }
            }
        }

        // Slices
        test_container(&arr_asc[..], Ord::Ascending);
        test_container(&arr_desc[..], Ord::Descending);

        // Mutable slices
        let mut arr_asc_mut = arr_asc;
        let mut arr_desc_mut = arr_desc;
        test_container(&mut arr_asc_mut[..], Ord::Ascending);
        test_container(&mut arr_desc_mut[..], Ord::Descending);

        // Vec
        let vec_asc = arr_asc.to_vec();
        let vec_desc = arr_desc.to_vec();
        test_container(vec_asc, Ord::Ascending);
        test_container(vec_desc, Ord::Descending);

        // Arrays
        test_container(arr_asc, Ord::Ascending);
        test_container(arr_desc, Ord::Descending);
        // Box

        let boxed_asc: Box<[i32]> = arr_asc.to_vec().into_boxed_slice();
        let boxed_desc: Box<[i32]> = arr_desc.to_vec().into_boxed_slice();
        test_container(boxed_asc, Ord::Ascending);
        test_container(boxed_desc, Ord::Descending);
    }
}
