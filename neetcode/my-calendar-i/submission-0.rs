#[derive(Debug)]
struct Event {
    start: i32,
    end: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

struct MyCalendar {    
    arena: Vec<Event>,
}


impl MyCalendar {
    fn new() -> Self {
        let head = Event {
            start: i32::MIN,
            end: i32::MIN,
            next: Some(1),
            prev: None,
        };
        let tail = Event {
            start: i32::MAX,
            end: i32::MAX,
            next: None,
            prev: Some(0),
        };
        let mut arena = Vec::new();
        arena.push(head);
        arena.push(tail);

        Self {
            arena,
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        let mut cur_idx = 0;
        let mut prev_idx = 0;
        let mut cur = &self.arena[cur_idx];
        while cur.end <= start {
            prev_idx = cur_idx;
            cur_idx = cur.next.unwrap();
            cur = &self.arena[cur_idx];
        }
        if end > cur.start {
            return false;
        }

        let new = Event {
            start,
            end,
            prev: Some(prev_idx),
            next: Some(cur_idx),
        };
        self.arena.push(new);
        let new_idx = self.arena.len() - 1;
        self.arena[prev_idx].next = Some(new_idx);
        self.arena[cur_idx].prev = Some(new_idx);

        println!("{:?}", self.arena);

        true
    }
}