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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize
    }
    let mut que: VecDeque<(i64, i64)> = VecDeque::new();
    let mut offset = 0i64;
    for _i in 0..n {
        input! {q:usize}
        if q == 1 {
            input! {l:i64}
            if let Some((pos, length)) = que.back() {
                que.push_back((*pos + *length, l));
            } else {
                que.push_back((0, l));
            }
        } else if q == 2 {
            let minus = que.pop_front().unwrap();
            offset += minus.1;
        } else {
            input! {k:Usize1}
            println!("{}", que[k].0 - offset);
        }
    }
}
fn main() {
    solve();
}
