#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct Index(usize);

#[derive(Clone, Copy, Debug)]
struct Cursor {
    prev: Option<Index>,
    next: Option<Index>,
}

#[derive(Default)]
pub struct ListContainer<T> {
    cursor: Vec<Cursor>,
    data: Vec<T>,
    free_list: Vec<usize>,
}

impl<T> ListContainer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cursor: Vec::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
            free_list: vec![],
        }
    }

    pub fn new_index(&mut self, datum: T) -> Index {
        if let Some(index) = self.free_list.pop() {
            let _ = std::mem::replace(&mut self.data[index], datum);
            Index(index)
        } else {
            self.data.push(datum);
            let cursor = Cursor {
                prev: None,
                next: None,
            };
            self.cursor.push(cursor);
            Index(self.data.len() - 1)
        }
    }

    fn setup_insertion(&mut self, at: Index, datum: T) -> (Index, Index) {
        let new_index = self.new_index(datum);
        (new_index, at)
    }

    fn finalize_insertion(
        &mut self,
        next: Index,
        next_value: Option<Index>,
        prev: Index,
        prev_value: Option<Index>,
    ) {
        self.set_next(next, next_value);
        self.set_prev(prev, prev_value);
    }

    fn set_next(&mut self, at: Index, value: Option<Index>) {
        self.cursor[at.0].next = value
    }

    fn set_prev(&mut self, at: Index, value: Option<Index>) {
        self.cursor[at.0].prev = value
    }

    pub fn add_list(&mut self, datum: T) -> Index {
        self.new_index(datum)
    }

    pub fn next(&self, at: Index) -> Option<Index> {
        self.cursor[at.0].next
    }

    pub fn prev(&self, at: Index) -> Option<Index> {
        self.cursor[at.0].prev
    }

    pub fn insert_after(&mut self, at: Index, datum: T) -> Index {
        let (new_index, index_at) = self.setup_insertion(at, datum);
        if let Some(next) = self.next(at) {
            self.set_next(new_index, Some(next));
            self.set_prev(next, Some(new_index));
        }
        self.finalize_insertion(index_at, Some(new_index), new_index, Some(index_at));
        new_index
    }

    pub fn insert_before(&mut self, at: Index, datum: T) -> Index {
        let (new_index, index_at) = self.setup_insertion(at, datum);
        if let Some(prev) = self.prev(at) {
            self.set_prev(new_index, Some(prev));
            self.set_next(prev, Some(new_index));
        }
        self.finalize_insertion(new_index, Some(index_at), index_at, Some(new_index));
        new_index
    }

    // Excise a node from a list.
    // The Index goes into the free list for
    // later recycling, making it a logic error
    // to use the value of `at` for further operations.
    pub fn remove(&mut self, at: Index) {
        let prev = self.prev(at);
        let next = self.next(at);

        if let Some(p) = prev {
            self.set_next(p, next);
        }
        if let Some(n) = next {
            self.set_prev(n, prev);
        }
        self.set_prev(at, None);
        self.set_next(at, None);
        self.free_list.push(at.0);
    }

    pub fn iterate_forward(&self, from: Index) -> impl Iterator<Item = &T> + '_ {
        IterateForward {
            next: Some(from),
            lists: self,
        }
    }

    pub fn iterate_backward(&self, from: Index) -> impl Iterator<Item = &T> + '_ {
        IterateBackward {
            prev: Some(from),
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

struct IterateBackward<'lists, T> {
    prev: Option<Index>,
    lists: &'lists ListContainer<T>,
}

impl<'lists, T> Iterator for IterateBackward<'lists, T> {
    type Item = &'lists T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.prev {
            self.prev = self.lists.cursor[index.0].prev;
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

    list.remove(head);
    let forward = list.iterate_forward(head).cloned().collect::<Vec<_>>();
    assert_eq!(
        forward,
        [0],
        "{:?}, {:?} {:?}, {:?}",
        list.cursor,
        head,
        next0,
        next1
    );
    let forward = list.iterate_forward(next0).cloned().collect::<Vec<_>>();
    assert_eq!(
        forward,
        [1, 7, 2],
        "{:?}, {:?} {:?}, {:?}",
        list.cursor,
        head,
        next0,
        next1
    );
    let forward = list.iterate_backward(next0).cloned().collect::<Vec<_>>();
    assert_eq!(
        forward,
        [1],
        "{:?}, {:?} {:?}, {:?}",
        list.cursor,
        head,
        next0,
        next1
    );
    let _ = list.insert_after(next0, 4);
    let forward = list.iterate_forward(next0).cloned().collect::<Vec<_>>();
    assert_eq!(
        forward,
        [1, 4, 7, 2],
        "{:?}, {:?} {:?}, {:?}",
        list.cursor,
        head,
        next0,
        next1
    );
}
