pub struct Queue<T> {
    inbox: Vec<T>,
    outbox: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            inbox: vec![],
            outbox: vec![],
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.inbox.push(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.fill_outbox();
        self.outbox.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if !self.outbox.is_empty() {
            self.outbox.last()
        } else {
            self.inbox.first()
        }
    }

    pub fn len(&self) -> usize {
        self.inbox.len() + self.outbox.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inbox.is_empty() && self.outbox.is_empty()
    }

    fn fill_outbox(&mut self) {
        if self.outbox.is_empty() {
            while let Some(e) = self.inbox.pop() {
                self.outbox.push(e);
            }
        }
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_peek() {
        let mut queue = Queue::new();
        for v in [10, 20, 30, 40, 50] {
            queue.enqueue(v);
        }

        assert_eq!(queue.peek(), Some(&10));
        assert_eq!(queue.len(), 5);

        _ = queue.dequeue();
        assert_eq!(queue.peek(), Some(&20));
        assert_eq!(queue.len(), 4);

        _ = queue.dequeue();
        assert_eq!(queue.peek(), Some(&30));
        assert_eq!(queue.len(), 3);

        _ = queue.dequeue();
        assert_eq!(queue.peek(), Some(&40));
        assert_eq!(queue.len(), 2);

        _ = queue.dequeue();
        assert_eq!(queue.peek(), Some(&50));
        assert_eq!(queue.len(), 1);

        _ = queue.dequeue();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());

        let dequeue_from_empty = queue.dequeue();
        assert_eq!(dequeue_from_empty, None);
    }

    #[test]
    fn test_dequeue_order() {
        // ------------------------------------------------------------------------

        let mut queue_a = Queue::new();
        let values_a = [0, 1, 2, 3, 4];
        for v in values_a {
            queue_a.enqueue(v);
        }
        assert_eq!(queue_a.len(), values_a.len());
        let expected_output_a = Vec::from(values_a);
        let mut output_a = vec![];
        while let Some(e) = queue_a.dequeue() {
            output_a.push(e);
        }
        assert_eq!(output_a, expected_output_a);

        // ------------------------------------------------------------------------

        let mut queue_b = Queue::new();
        let values_b = [0, 1, 2, 3];
        for v in values_b {
            queue_b.enqueue(v);
        }
        assert_eq!(queue_b.len(), values_b.len());
        let mut i = 0;
        while let Some(e) = queue_b.dequeue() {
            assert_eq!(e, values_b[i]);
            i += 1;
        }

        // ------------------------------------------------------------------------
    }
}
