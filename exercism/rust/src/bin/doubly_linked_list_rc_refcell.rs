use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

// ---------- Node ----------
type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    prev: WeakLink<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

// ---------- LinkedList ----------
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let current = self.front.clone();
        Cursor { list: self, current }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let current = self.back.clone();
        Cursor { list: self, current }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.front.clone(),
        }
    }
}

// ---------- Cursor ----------
pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: Link<T>,
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<RefMut<'_, T>> {
        let node = self.current.as_ref()?;
        Some(RefMut::map(node.borrow_mut(), |n| &mut n.value))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<RefMut<'_, T>> {
        let next = self.current.as_ref()?.borrow().next.clone();
        self.current = next;
        self.peek_mut()
    }

    pub fn prev(&mut self) -> Option<RefMut<'_, T>> {
        let prev = self
            .current
            .as_ref()?
            .borrow()
            .prev
            .clone()
            .and_then(|w| w.upgrade());
        self.current = prev;
        self.peek_mut()
    }

    pub fn take(&mut self) -> Option<T> {
        let node_rc = self.current.take()?;

        let prev_weak = node_rc.borrow().prev.clone();
        let next_rc = node_rc.borrow().next.clone();
        let prev_rc = prev_weak.as_ref().and_then(|w| w.upgrade());

        match &prev_rc {
            Some(p) => p.borrow_mut().next = next_rc.clone(),
            None => self.list.front = next_rc.clone(),
        }
        match &next_rc {
            Some(n) => n.borrow_mut().prev = prev_weak.clone(),
            None => self.list.back = prev_rc.clone(),
        }

        // giữ đúng ngữ nghĩa bản arena: sau khi xoá, current trỏ tới next, nếu không có thì prev
        self.current = next_rc.or(prev_rc);
        self.list.len -= 1;

        // lúc này không còn strong ref nào khác trỏ tới node -> try_unwrap phải thành công
        match Rc::try_unwrap(node_rc) {
            Ok(cell) => Some(cell.into_inner().value),
            Err(_) => unreachable!("còn strong reference khác trỏ tới node vừa xoá"),
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let new_node = Node::new(element);
        match self.current.clone() {
            None => {
                self.list.front = Some(new_node.clone());
                self.list.back = Some(new_node.clone());
                self.current = Some(new_node);
            }
            Some(cur) => {
                let next = cur.borrow().next.clone();

                new_node.borrow_mut().prev = Some(Rc::downgrade(&cur));
                new_node.borrow_mut().next = next.clone();

                cur.borrow_mut().next = Some(new_node.clone());
                match next {
                    Some(n) => n.borrow_mut().prev = Some(Rc::downgrade(&new_node)),
                    None => self.list.back = Some(new_node),
                }
            }
        }
        self.list.len += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        let new_node = Node::new(element);
        match self.current.clone() {
            None => {
                self.list.front = Some(new_node.clone());
                self.list.back = Some(new_node.clone());
                self.current = Some(new_node);
            }
            Some(cur) => {
                let prev = cur.borrow().prev.clone().and_then(|w| w.upgrade());

                new_node.borrow_mut().next = Some(cur.clone());
                new_node.borrow_mut().prev = prev.clone().map(|p| Rc::downgrade(&p));

                cur.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                match prev {
                    Some(p) => p.borrow_mut().next = Some(new_node),
                    None => self.list.front = Some(new_node),
                }
            }
        }
        self.list.len += 1;
    }
}

// ---------- Iter ----------
pub struct Iter<T> {
    current: Link<T>,
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let node = self.current.take()?;
        let next = node.borrow().next.clone();
        self.current = next;
        let value = node.borrow().value.clone();
        Some(value)
    }
}

fn main() {}
