type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T, 
    next: Link<T>,
}

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        !&self.head.is_some()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut cur = self.head.as_deref();

        while let Some(node) = cur {
            count += 1;
            cur = node.next.as_deref();
        }

        count
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut prev = None;
        let mut cur = self.head.take();

        while let Some(mut node) = cur {
            let next = node.next.take();

            node.next = prev;
            prev = Some(node);
            cur = next;
        }

        self.head = prev;
        self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for elem in iter {
            list.push(elem);        
        }

        list
    }
}


impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();

        while let Some(elem)= linked_list.pop() {
            vec.push(elem);
        }

        vec.reverse();
        vec
    }
}
