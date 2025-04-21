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
        n:usize,mut a:[usize;n]
    }
    let mut total = 0;
    a.sort();
    let mut mapped = (0..n)
        .into_iter()
        .map(|i| (a[i] + a[0]).rem_euclid(1e8 as usize))
        .enumerate()
        .collect::<Vec<(usize, usize)>>();
    mapped[0].1 = 0;
    mapped.sort_by(|a, b| a.1.cmp(&b.1));
    let mut sum = mapped.iter().map(|(x, y)| *y).sum::<usize>();
    println!("{}", sum);
    for i in 0..n {
        print!("(idx:{},val:{}) ", mapped[i].0, mapped[i].1);
    }
    println!();
    total += sum;
    for i in 1..n - 1 {
        sum -= mapped[i].1;
        let diff = a[mapped[i].0] - a[0];
        let overs = (n - idx) * 1e8 as usize;
        dbg!(&diff, &idx, &overs, &sum);
        total += sum - overs;
    }
    // println!("{}", total);
}
fn main() {
    solve();
}
