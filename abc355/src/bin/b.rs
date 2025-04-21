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
        n:usize,m:usize,a:[usize;n],b:[usize;m]
    }
    let a = a.iter().map(|x| (*x, 1)).collect::<Vec<(usize, i32)>>();
    let b = b.iter().map(|x| (*x, 2)).collect::<Vec<(usize, i32)>>();
    let mut ans = Vec::new();
    for i in 0..n {
        ans.push(a[i]);
    }
    for i in 0..m {
        ans.push(b[i]);
    }
    ans.sort();
    for i in 0..n + m - 1 {
        if ans[i].1 == 1 && ans[i + 1].1 == 1 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
fn main() {
    solve();
}
