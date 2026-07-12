struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

struct Queue<T> {
    last: *const Node<T>,
    first: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            item: value,
            next: None,
        }
    }
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            last: std::ptr::null(),
            first: None,
        }
    }

    fn push(&mut self, value: T) {
        unsafe {
            if let Some(r_last) = &mut self.last.as_ref() {
                assert!(r_last.next.is_none());
                let node = Box::new(Node {
                    item: value,
                    next: None,
                });
                (*r_last).next = Some(node);
            } else {
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_queue() {
        let _new_queue: Queue<i32> = Queue::new();
    }
}
