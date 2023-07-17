#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct Index(usize);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct Head(usize);

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    current: Index,
    prev: Option<Index>,
    next: Option<Index>,
}

#[derive(Default)]
pub struct ListContainer<T> {
    //head: Vec<Index>,
    cursor: Vec<Cursor>,
    data: Vec<T>,
    free_list: Vec<usize>,
}

impl<T> ListContainer<T> {
    pub fn new_cursor(&mut self, datum: T) -> Cursor {
        if let Some(index) = self.free_list.pop() {
            let _ = std::mem::replace(&mut self.data[index], datum);
            let cursor = Cursor {
                current: Index(index),
                prev: None,
                next: None,
            };
            self.cursor[index] = cursor;
            cursor
        } else {
            self.data.push(datum);
            let cursor = Cursor {
                current: Index(self.data.len() - 1),
                prev: None,
                next: None,
            };
            self.cursor.push(cursor);
            cursor
        }
    }

    pub fn add_list(&mut self, datum: T) -> Cursor {
        self.new_cursor(datum)
    }

    pub fn insert_after(&mut self, cursor: &mut Cursor, datum: T) -> Cursor {
        let mut new_cursor = self.new_cursor(datum);
        let mut internal_cursor = self.cursor[cursor.current.0];
        if let Some(next) = internal_cursor.next {
            internal_cursor.next = Some(new_cursor.current);
            new_cursor.prev = Some(internal_cursor.current);
            new_cursor.next = Some(next);
            self.cursor[next.0].prev = Some(new_cursor.current);
        } else {
            internal_cursor.next = Some(new_cursor.current);
            new_cursor.prev = Some(internal_cursor.current);
        }
        *cursor = internal_cursor;
        self.cursor[cursor.current.0] = internal_cursor;
        self.cursor[new_cursor.current.0] = new_cursor;
        new_cursor
    }

    pub fn insert_before(&mut self, cursor: &mut Cursor, datum: T) -> Cursor
    {
        let mut new_cursor = self.new_cursor(datum);
        let mut internal_cursor = self.cursor[cursor.current.0];
        if let Some(prev) = internal_cursor.prev {
            internal_cursor.prev = Some(new_cursor.current);
            new_cursor.next = Some(internal_cursor.current);
            new_cursor.prev = Some(prev);
            self.cursor[prev.0].next = Some(new_cursor.current);
        } else {
            internal_cursor.prev = Some(new_cursor.current);
            new_cursor.next = Some(internal_cursor.current);
        }
        *cursor = internal_cursor;
        self.cursor[cursor.current.0] = internal_cursor;
        self.cursor[new_cursor.current.0] = new_cursor;
        new_cursor
    }

    pub fn iterate_forward(&self, from: Cursor) -> impl Iterator<Item = &T> + '_ {
        IterateForward {
            next: Some(from.current),
            lists: self,
        }
    }
}

struct IterateForward<'lists, T> {
    next: Option<Index>,
    lists: &'lists ListContainer<T>,
}

impl<'lists, T> Iterator for IterateForward<'lists, T> {
    type Item = &'lists T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.next {
            self.next = self.lists.cursor[index.0].next;
            Some(&self.lists.data[index.0])
        } else {
            None
        }
    }
}

#[test]
fn test_insert_after() {
    let mut list = ListContainer::<i32>::default();
    let mut head = list.add_list(0);
    let mut next0 = list.insert_after(&mut head, 1);
    let mut next1 = list.insert_after(&mut next0, 2);

    assert_eq!(list.data[head.current.0], 0);
    assert_eq!(list.data[next0.current.0], 1);
    assert_eq!(list.data[next1.current.0], 2);

    for i in &list.cursor {
        println!("before {i:?}");
    }

    let forward = list.iterate_forward(head).cloned().collect::<Vec<_>>();
    assert_eq!(
        forward,
        [0, 1, 2],
        "{:?}, {:?} {:?}, {:?}",
        list.cursor,
        head,
        next0,
        next1
    );

    let _ = list.insert_before(&mut next1, 7);
    for i in &list.cursor {
        println!("after {i:?}");
    }
    let forward = list.iterate_forward(head).cloned().collect::<Vec<_>>();
    assert_eq!(
        forward,
        [0, 1, 7, 2],
        "{:?}, {:?} {:?}, {:?}",
        list.cursor,
        head,
        next0,
        next1
    );
}
