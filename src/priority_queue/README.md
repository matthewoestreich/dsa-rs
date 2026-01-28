# PriorityQueue

[You can read more on priority queue here](https://en.wikipedia.org/wiki/Priority_queue).

This implementation uses a binary heap for storing data.

You set the priority of `T` via the `compare` function. The compare result is relative to `a`.

# Examples

## Minimum Priority Queue

Elements that are ordered `Ordering::Less` have a higher priority.

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
    let compare = |a: &User, b: &User| {
        // Users with a lower `sign_in_count` have a higher priority.
        let sign_in_count = b.sign_in_count.cmp(&a.sign_in_count);

        // If two users have the same `sign_in_count` values,
        // fall back to the `is_active` field.
        if sign_in_count == Ordering::Equal {
            let is_active = b.is_active.cmp(&a.is_active);

            // If two users have the same `sign_in_count` AND `is_active`
            // values, use sorting usernames as final fall back.
            if is_active == Ordering::Equal {
                return b.username.cmp(&a.username);
            }

            // Return first `is_active` fall back.
            return is_active;
        }

        // Return original `sign_in_count` comparison.
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
```

## Maximum Priority Queue

Elements that are ordered `Ordering::Greater` have a higher priority.

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
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
```
