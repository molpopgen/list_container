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
    pub fn new_index(&mut self, datum: T) -> Index {
        if let Some(index) = self.free_list.pop() {
            let _ = std::mem::replace(&mut self.data[index], datum);
            Index(index)
        } else {
            self.data.push(datum);
            let cursor = Cursor {
                current: Index(self.data.len() - 1),
                prev: None,
                next: None,
            };
            self.cursor.push(cursor);
            Index(self.data.len() - 1)
        }
    }

    pub fn add_list(&mut self, datum: T) -> Index {
        self.new_index(datum)
    }

    pub fn insert_after(&mut self, at: Index, datum: T) -> Index {
        let new_index = self.new_index(datum);
        let mut internal_cursor = self.cursor[at.0];
        if let Some(next) = internal_cursor.next {
            internal_cursor.next = Some(new_index);
            let new_cursor = &mut self.cursor[new_index.0];
            new_cursor.prev = Some(internal_cursor.current);
            new_cursor.next = Some(next);
            self.cursor[next.0].prev = Some(new_index);
        } else {
            internal_cursor.next = Some(new_index);
            let new_cursor = &mut self.cursor[new_index.0];
            new_cursor.prev = Some(internal_cursor.current);
        }
        self.cursor[internal_cursor.current.0] = internal_cursor;
        new_index
    }

    pub fn insert_before(&mut self, at: Index, datum: T) -> Index {
        let new_index = self.new_index(datum);
        let mut internal_cursor = self.cursor[at.0];
        if let Some(prev) = internal_cursor.prev {
            internal_cursor.prev = Some(new_index);
            let new_cursor = &mut self.cursor[new_index.0];
            new_cursor.next = Some(internal_cursor.current);
            new_cursor.prev = Some(prev);
            self.cursor[prev.0].next = Some(new_index);
        } else {
            internal_cursor.prev = Some(new_index);
            let new_cursor = &mut self.cursor[new_index.0];
            new_cursor.next = Some(internal_cursor.current);
        }
        self.cursor[internal_cursor.current.0] = internal_cursor;
        //self.cursor[internal_cursor.current.0] = new_index;
        new_index
    }

    pub fn iterate_forward(&self, from: Index) -> impl Iterator<Item = &T> + '_ {
        IterateForward {
            next: Some(from),
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
    let head = list.add_list(0);
    let next0 = list.insert_after(head, 1);
    let next1 = list.insert_after(next0, 2);

    assert_eq!(list.data[head.0], 0);
    assert_eq!(list.data[next0.0], 1);
    assert_eq!(list.data[next1.0], 2);

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

    let _ = list.insert_before(next1, 7);
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
