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
        s:Chars
    }
    let mut cnt: HashMap<char, usize> = HashMap::new();
    for i in 0..s.len() {
        *cnt.entry(s[i]).or_insert(0) += 1;
    }
    let mut ans = HashMap::new();
    for (k, v) in cnt {
        ans.entry(v).or_insert(Vec::new()).push(k);
    }
    for (k, v) in ans {
        if v.len() != 0 && v.len() != 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
