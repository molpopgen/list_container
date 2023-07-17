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
            cursor = list.insert_after(cursor, i);
        }
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
            cursor = list.insert_after(cursor, i);
        }
    });
}

fn add_1000_in_two_lists(bench: &mut Bencher) {
    bench.iter(|| {
        let mut list = ListContainer::<i32>::default();
        let head = list.add_list(0);
        let head2 = list.add_list(0);
        let mut cursor = head;
        let mut cursor2 = head2;
        for i in 1..500 {
            cursor = list.insert_after(cursor, i);
            cursor2 = list.insert_after(cursor2, i);
        }
    });
}

fn add_1000_in_two_lists_with_capacity(bench: &mut Bencher) {
    bench.iter(|| {
        let mut list = ListContainer::<i32>::with_capacity(1000);
        let head = list.add_list(0);
        let head2 = list.add_list(0);
        let mut cursor = head;
        let mut cursor2 = head2;
        for i in 1..500 {
            cursor = list.insert_after(cursor, i);
            cursor2 = list.insert_after(cursor2, i);
        }
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

fn vec_1000_no_capacity(bench: &mut Bencher) {
    bench.iter(|| {
        let mut v = vec![];
        for i in 0..1000 {
            std::hint::black_box(v.push(i));
        }
    })
}

fn vec_1000_with_capacity(bench: &mut Bencher) {
    bench.iter(|| {
        let mut v =Vec::with_capacity(1000);
        for i in 0..1000 {
            std::hint::black_box(v.push(i));
        }
    })
}

benchmark_group!(
    bench,
    add_100,
    add_100_linked_list,
    add_1000,
    add_1000_in_two_lists,
    add_1000_in_two_lists_with_capacity,
    add_1000_linked_list,
    vec_1000_no_capacity,
    vec_1000_with_capacity,
);

benchmark_main!(bench);
