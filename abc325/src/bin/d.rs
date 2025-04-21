use btreemultimap::BTreeMultiMap;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::hash::Hash;
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[derive(Debug, Clone)]
pub struct ReversalBinaryHeap<T>
where
    T: Copy + Clone + Hash + Eq + Ord,
{
    heap: BinaryHeap<Reverse<T>>,
}

impl<T> ReversalBinaryHeap<T>
where
    T: Copy + Clone + Hash + Eq + Ord,
{
    pub fn new() -> Self {
        let heap: BinaryHeap<Reverse<T>> = BinaryHeap::new();
        ReversalBinaryHeap { heap: heap }
    }
    pub fn push(&mut self, x: T) {
        self.heap.push(Reverse(x));
    }
    pub fn pop(&mut self) -> Option<T> {
        let val = self.heap.pop();
        let x = match val {
            Some(val) => Some(val.0),
            None => None,
        };
        x
    }
    pub fn from(v: Vec<T>) -> Self {
        let mut heap: BinaryHeap<Reverse<T>> = BinaryHeap::new();
        for val in &v {
            heap.push(Reverse(*val));
        }
        ReversalBinaryHeap { heap: heap }
    }
    pub fn clear(&mut self) {
        self.heap.clear();
    }
    pub fn len(&mut self) -> usize {
        self.heap.len()
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut items:[(usize,usize);n]
    }
    let mut que: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut now = 0;
    let mut ans = 0;
    items.sort();
    let idx = 1;
    que.push(Reverse(items[0].1));
    while let Some(p) = que.peek() {
        let end = p.0;
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
