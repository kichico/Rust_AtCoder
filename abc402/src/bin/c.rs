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
        n:usize,m:usize
    }
    let mut shokuzai2dish: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for i in 0..m {
        input! {k:usize,a:[usize;k]}
        for j in 0..k {
            shokuzai2dish
                .entry(a[j])
                .or_insert(BTreeSet::new())
                .insert(i);
        }
    }
    input! { mut b:[usize;n] }
    b.reverse();
    let mut ans = vec![0; n];
    let mut rest = (0..m).into_iter().collect::<BTreeSet<usize>>();
    for i in 0..n {
        ans[i] = rest.len();
        if let Some(shokuzai_st) = shokuzai2dish.get(&b[i]) {
            for shokuzai in shokuzai_st {
                rest.remove(shokuzai);
            }
        }
    }
    ans.reverse();
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
