#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::hash::Hash;
use std::iter::FromIterator;
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,q:usize,a:[i64;n],query:[(i32,usize,usize);q]
    }
    let mut deq: VecDeque<i64> = VecDeque::from_iter(a.into_iter());
    for (t, x, y) in query {
        if t == 1 {
            let x = x - 1;
            let y = y - 1;
            let xv = deq[x].clone();
            let yv = deq[y].clone();
            deq[y] = xv;
            deq[x] = yv;
        } else if t == 2 {
            let front = deq.pop_back().unwrap();
            deq.push_front(front);
        } else {
            let x = x - 1;
            println!("{}", deq[x]);
        }
    }
}
fn main() {
    solve();
}
