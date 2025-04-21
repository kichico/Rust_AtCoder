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
        n:usize,k:usize,a:[usize;n]
    }
    let mut dic: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        dic.insert(a[i], i + 1);
    }

    let mut ranges = BTreeSet::from_iter((1..=k).into_iter().map(|x| *dic.get(&x).unwrap()));
    let mut ans = ranges.last().unwrap() - ranges.first().unwrap();
    for i in k + 1..=n {
        ranges.remove(dic.get(&(i - k)).unwrap());
        ranges.insert(*dic.get(&i).unwrap());
        ans = ans.min(ranges.last().unwrap() - ranges.first().unwrap());
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
