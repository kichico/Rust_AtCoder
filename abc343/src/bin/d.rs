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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,t:usize,event:[(Usize1,usize);t]
    }
    let mut score = vec![0; n];
    let mut cnt: HashMap<usize, usize> = HashMap::new();
    cnt.insert(0, n);
    for (p, s) in event {
        let x = cnt.get_mut(&score[p]).unwrap();
        *x -= 1;
        if x == &0 {
            cnt.remove(&score[p]);
        }
        score[p] += s;
        *cnt.entry(score[p]).or_insert(0) += 1;
        println!("{}", cnt.len());
    }
}
fn main() {
    solve();
}
