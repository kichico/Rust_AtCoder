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
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,s:usize,m:usize,l:usize
    }
    let mut ans = 1e18 as usize;
    for ss in 0..20 {
        for mm in 0..14 {
            for ll in 0..10 {
                if ss * 6 + mm * 8 + ll * 12 < n {
                    continue;
                }
                ans = ans.min(ss * s + mm * m + ll * l);
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
