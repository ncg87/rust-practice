// Make the code compile by implementing the Iterator trait for `Queue`.

struct Queue {
    items: Vec<i32>,
}

impl Queue {
    fn new(items: Vec<i32>) -> Self {
        Self { items }
    }
}

impl Iterator for Queue {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return None;
        }
        Some(self.items.remove(0))
    }
}

fn main() {
    let mut queue = Queue::new(vec![3, 2, 1]);
    assert!(matches!(queue.next(), Some(3)));
    assert!(matches!(queue.next(), Some(2)));
    assert!(matches!(queue.next(), Some(1)));
    assert!(matches!(queue.next(), None));
}
