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
use std::iter::{Iterator, Rev};
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
    pub fn peek(&self) -> Option<&T> {
        let ret = match self.heap.peek() {
            Some(x) => Some(&x.0),
            None => None,
        };
        ret
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,slimes:[(usize,usize);n]
    }
    let mut mp: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    let mut ans = 0;
    for (val, num) in slimes {
        mp.push((Reverse(val), num));
    }
    let mut db: Vec<usize> = Vec::new();
    while let Some(val) = mp.pop() {
        let mut cnt = val.1;
        while mp.len() != 0 && mp.peek().unwrap().0 == val.0 {
            let p = mp.pop().unwrap().1;
            cnt += p;
        }
        let plus = cnt / 2;
        cnt -= plus * 2;
        if plus > 0 {
            mp.push((Reverse(val.0 .0 * 2), plus));
        }
        if cnt == 1 {
            ans += 1;
            db.push(val.0 .0);
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
