# Heap

Binary heap implementation. [You can read more about binary heaps here](https://en.wikipedia.org/wiki/Binary_heap).

# Examples

## Minimum Heap

```rust
#[test]
fn test_min_heap() {
    let compare = |a: &i32, b: &i32| b.cmp(a);
    let values = vec![30, 20, 90, 50, 60, 10];
    let min_heap = Heap::new(compare, Some(values));

    let heap_to_sorted = min_heap.to_sorted_vec();
    let expected_sort = vec![10, 20, 30, 50, 60, 90];
    assert_eq!(heap_to_sorted, expected_sort);

    assert_eq!(min_heap.front().expect("some"), &10);
    assert_eq!(min_heap.leaf().expect("some"), &90);
}
```

## Maximum Heap

```rust
#[test]
fn test_max_heap() {
    let compare = |a: &i32, b: &i32| a.cmp(b);
    let values = vec![30, 20, 90, 50, 60, 10];
    let max_heap = Heap::new(compare, Some(values));

    let heap_to_sorted = max_heap.to_sorted_vec();
    let expected_sort = vec![90, 60, 50, 30, 20, 10];
    assert_eq!(heap_to_sorted, expected_sort);

    assert_eq!(max_heap.front().expect("some"), &90);
    assert_eq!(max_heap.leaf().expect("some"), &10);
}
```
