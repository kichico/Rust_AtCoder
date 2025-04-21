#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use omniswap::swap;
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
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,q:usize,color:[usize;n],query:[(Usize1,Usize1);q]
    }
    let mut boxes: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for i in 0..n {
        boxes[i].insert(color[i]);
    }
    for (from, to) in query {
        let mut f = from.clone();
        let mut t = to.clone();
        if boxes[from].len() > boxes[to].len() {
            swap(&mut f, &mut t);
        }
        let smaller = std::mem::take(&mut boxes[f]);
        for c in smaller {
            boxes[t].insert(c);
        }
        if from != f {
            let v = std::mem::take(&mut boxes[from]);
            boxes[to] = v;
        }
        println!("{}", boxes[to].len());
    }
}
fn main() {
    solve();
}
