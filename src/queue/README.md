# Queue

Implement a queue (FIFO) using two stacks.

```rust
fn main() {
    let mut queue = Queue::new();

    let values = [0, 1, 2, 3];
    for v in values {
        queue.enqueue(v);
    }

    let mut i = 0;
    while let Some(e) = queue.dequeue() {
        assert_eq!(e, values[i]);
        i += 1;
    }
}
```
