# PriorityQueue

You set the priority of `T` via the `comparator` function.

# Examples

## Minimum Priority Queue

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
    let min_priority_comparator = |a: &User, b: &User| {
        // First, we target the `sign_in_count` field.
        // Users with a lower `sign_in_count` have a higher priority.
        let sic_result = a.sign_in_count.cmp(&b.sign_in_count);

        // If two users have the same `sign_in_count`, we use `is_active` field as fallback.
        if sic_result == Ordering::Equal {
            // Essentially, we want `true` to rank "higher" than `false` in a min queue.
            // Traditionally, in a min queue, `false` would hold a higher priority than `true`.
            // Therefore, we use `b` to compare to `a` here.
            let ia_result = b.is_active.cmp(&a.is_active);

            // If two users have the same `sign_in_count` AND `is_active` values,
            // we use `username` field as final fallback.
            if ia_result == Ordering::Equal {
                return a.username.cmp(&b.username);
            }

            // Return `is_active` comparison as first fallback.
            return ia_result;
        }

        // Return `sign_in_count` comparison.
        sic_result
    };

    // Values we will use in our min priority queue.
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
        User::new("sybil_2", "sybil_2@example.com", 1, false),
        User::new("trent", "trent@example.com", 76, true),
        User::new("ursula", "ursula@example.com", 9, false),
        User::new("victor", "victor@example.com", 100, true),
    ];

    // Create queue.
    let mut min_priority_queue = PriorityQueue::new(min_priority_comparator, Some(user_values));

    // `front()` holds element with "highest" priority.
    // Since this is a min queue, "highest" priority means "lowest" ordering.
    assert_eq!(
        min_priority_queue.front().expect("some"),
        &User::new("sybil", "sybil@example.com", 1, false)
    );

    // `back()` is opposite of `front()`.
    assert_eq!(
        min_priority_queue.back().expect("some"),
        &User::new("victor", "victor@example.com", 100, true)
    );
}
```
