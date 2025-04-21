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
        t:usize
    }
    let mut allans: Vec<usize> = Vec::new();
    for _ in 0..t {
        input! {n:usize,k:usize,mut a:[usize;n],mut b:[usize;n]}
        let comb = a.iter().zip(b).sorted().collect::<Vec<_>>();
        let (a, b): (Vec<_>, Vec<_>) = comb.iter().map(|(x, y)| (**x, *y)).unzip();
        let mut que = BinaryHeap::from_iter(b.iter().take(k - 1));
        let mut ans = usize::MAX;
        let mut bsum: usize = b.iter().take(k - 1).sum();
        for i in k - 1..n {
            //println!("a[{}]:{} bsum:{} b[{}]:{}", i, a[i], bsum, i, b[i]);
            ans = ans.min(a[i] * (bsum + b[i]));
            que.push(&b[i]);
            bsum += b[i];
            bsum -= que.pop().unwrap();
            //println!("que:{}", que.iter().join(" "));
        }
        allans.push(ans);
    }
    println!("{}", allans.iter().join("\n"));
}
fn main() {
    solve();
}
