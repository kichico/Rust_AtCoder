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
        n:usize,a:[Usize1;n-1]
    }
    let mut cnt = 1;
    let mut a: VecDeque<&usize> = a.iter().collect::<VecDeque<_>>();
    let mut now = n - 1;
    a.push_front(&n);
    while *a[now] != 0 {
        cnt += 1;
        now = *a[now]
    }
    println!("{}", cnt);
}

fn main() {
    solve();
}
