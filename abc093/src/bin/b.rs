#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        a:i64,b:i64,k:i64
    }
    let mut ans: BTreeSet<i64> = BTreeSet::new();
    for i in 0..k {
        if a + i <= b {
            ans.insert(a + i);
        }
        if b - i >= a {
            ans.insert(b - i);
        }
    }
    for c in ans {
        println!("{}", c);
    }
}
fn main() {
    solve();
}
