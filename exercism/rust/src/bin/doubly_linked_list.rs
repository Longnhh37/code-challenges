use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

pub struct Cursor<'a, T> {
    current: Link<T>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    next: Link<T>,
    _marker: std::marker::PhantomData<&'a T>,
}

#[allow(clippy::new_without_default)]
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
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
        Cursor {
            current: self.head.clone(),
            list: self,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.tail.clone(),
            list: self,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.clone(),
            _marker: std::marker::PhantomData,

        }
    }
}

impl<'a, T> Cursor<'a, T> {
    pub fn peek_mut(&mut self) -> Option<RefMut<'_, T>> {
        self.current.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |n| &mut n.elem)
        })
    }
}

fn main() {}
