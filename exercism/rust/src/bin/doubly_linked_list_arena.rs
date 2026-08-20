// ---------- Arena node ----------

struct Node<T> {
    value: T,
    prev: Option<usize>,
    next: Option<usize>,
}

enum Slot<T> {
    Occupied(Node<T>),
    Free(Option<usize>),
}

// ---------- LinkedList ----------

pub struct LinkedList<T> {
    arena: Vec<Slot<T>>,
    front: Option<usize>,
    back: Option<usize>,
    free_head: Option<usize>,
    len: usize,
}

impl<T> LinkedList<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            arena: Vec::new(),
            front: None,
            back: None,
            free_head: None,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn alloc(&mut self, node: Node<T>) -> usize {
        self.len += 1;
        match self.free_head {
            None => {
                self.arena.push(Slot::Occupied(node));
                self.arena.len() - 1
            }
            Some(idx) => {
                self.free_head = match self.arena[idx] {
                    Slot::Free(next) => next,
                    Slot::Occupied(_) => unreachable!(),
                };
                self.arena[idx] = Slot::Occupied(node);
                idx
            }
        }
    }

    fn dealloc(&mut self, idx: usize) -> T {
        let old_free_head = self.free_head;
        let slot = std::mem::replace(&mut self.arena[idx], Slot::Free(old_free_head));
        let node = match slot {
            Slot::Occupied(node) => node,
            Slot::Free(_) => unreachable!(),
        };
        self.free_head = Some(idx);
        self.len -= 1;
        node.value
    }

    fn node(&self, idx: usize) -> &Node<T> {
        match &self.arena[idx] {
            Slot::Occupied(node) => node,
            Slot::Free(_) => unreachable!(),
        }
    }

    fn node_mut(&mut self, idx: usize) -> &mut Node<T> {
        match &mut self.arena[idx] {
            Slot::Occupied(node) => node,
            Slot::Free(_) => unreachable!(),
        }
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let current = self.front;
        Cursor {
            list: self,
            current,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let current = self.back;
        Cursor {
            list: self,
            current,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            current: self.front,
        }
    }
}

// ---------- Cursor ----------

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: Option<usize>,
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let idx = self.current?;
        Some(&mut self.list.node_mut(idx).value)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        let idx = self.current?;
        let next_idx = self.list.node(idx).next;
        self.current = next_idx;
        self.peek_mut()
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        let idx = self.current?;
        let prev_idx = self.list.node(idx).prev;
        self.current = prev_idx;
        self.peek_mut()
    }

    pub fn take(&mut self) -> Option<T> {
        let idx = self.current?;
        let (prev, next) = {
            let node = self.list.node(idx);
            (node.prev, node.next)
        };

        match prev {
            Some(p) => self.list.node_mut(p).next = next,
            None => self.list.front = next,
        }
        match next {
            Some(n) => self.list.node_mut(n).prev = prev,
            None => self.list.back = prev,
        }

        self.current = match next {
            Some(n) => Some(n),
            None => prev,
        };

        Some(self.list.dealloc(idx))
    }

    pub fn insert_after(&mut self, element: T) {
        match self.current {
            None => {
                let idx = self.list.alloc(Node {
                    value: element,
                    prev: None,
                    next: None,
                });
                self.list.front = Some(idx);
                self.list.back = Some(idx);
                self.current = Some(idx);
            }
            Some(idx) => {
                let next = self.list.node(idx).next;
                let new_idx = self.list.alloc(Node {
                    value: element,
                    prev: Some(idx),
                    next,
                });
                self.list.node_mut(idx).next = Some(new_idx);
                match next {
                    Some(n) => self.list.node_mut(n).prev = Some(new_idx),
                    None => self.list.back = Some(new_idx),
                }
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        match self.current {
            None => {
                let idx = self.list.alloc(Node {
                    value: element,
                    prev: None,
                    next: None,
                });
                self.list.front = Some(idx);
                self.list.back = Some(idx);
                self.current = Some(idx);
            }
            Some(idx) => {
                let prev = self.list.node(idx).prev;
                let new_idx = self.list.alloc(Node {
                    value: element,
                    prev,
                    next: Some(idx),
                });
                self.list.node_mut(idx).prev = Some(new_idx);
                match prev {
                    Some(p) => self.list.node_mut(p).next = Some(new_idx),
                    None => self.list.front = Some(new_idx),
                }
            }
        }
    }
}

// ---------- Iter ----------

pub struct Iter<'a, T> {
    list: &'a LinkedList<T>,
    current: Option<usize>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let idx = self.current?;
        let node = self.list.node(idx);
        self.current = node.next;
        Some(&node.value)
    }
}

fn main() {}
