#[macro_use]
extern crate bencher;

use bencher::Bencher;
use std::collections::LinkedList;

use list_container::*;

fn add_100(bench: &mut Bencher) {
    bench.iter(|| {
        let mut list = ListContainer::<i32>::default();
        let head = list.add_list(0);
        let mut cursor = head;
        for i in 1..100 {
            cursor = list.insert_after(&mut cursor, i);
        }
        //let _ = list.iterate_forward(head).cloned().collect::<Vec<_>>();
    });
}

fn add_100_linked_list(bench: &mut Bencher) {
    bench.iter(|| {
        let mut list: LinkedList<u32> = LinkedList::new();
        for i in 0..100 {
            list.push_back(i);
        }
    });
}

fn add_1000(bench: &mut Bencher) {
    bench.iter(|| {
        let mut list = ListContainer::<i32>::default();
        let head = list.add_list(0);
        let mut cursor = head;
        for i in 1..1000 {
            cursor = list.insert_after(&mut cursor, i);
        }
        //let _ = list.iterate_forward(head).cloned().collect::<Vec<_>>();
    });
}

fn add_1000_linked_list(bench: &mut Bencher) {
    bench.iter(|| {
        let mut list: LinkedList<u32> = LinkedList::new();
        for i in 0..1000 {
            list.push_back(i);
        }
    });
}

benchmark_group!(
    bench,
    add_100,
    add_100_linked_list,
    add_1000,
    add_1000_linked_list
);

benchmark_main!(bench);
