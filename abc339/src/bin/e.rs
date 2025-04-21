use ac_library::{segtree, Max};
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
        n:usize,d:usize,a:[usize;n]
    }
    let maxi = *a.iter().max().unwrap();
    let mut dp = segtree::Segtree::<ac_library::Max<usize>>::new(maxi + 1);
    for i in 1..=maxi {
        dp.set(i, 0);
    }
    for i in 0..n {
        let lower = a[i].saturating_sub(d).clamp(1, maxi);
        let upper = (a[i] + d).clamp(1, maxi);
        let current_max = dp.prod(lower..=upper);
        dp.set(a[i], current_max + 1);
    }
    println!("{}", dp.all_prod());
}
fn main() {
    solve();
}
