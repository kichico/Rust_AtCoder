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
        n:usize,foods:[usize;n],ra:[usize;n],rb:[usize;n]
    }
    let mut maxi_a = 1e18 as usize;
    for i in 0..n {
        if ra[i] == 0 {
            continue;
        }
        maxi_a = maxi_a.min(foods[i] / ra[i]);
    }
    let mut ans = 0;
    'outer: for pa in 0..=maxi_a {
        let mut f = foods.clone();
        for i in 0..n {
            if f[i] < ra[i] * pa {
                continue 'outer;
            }
            f[i] -= ra[i] * pa;
        }
        let mut maxi_b = 1e18 as usize;
        for i in 0..n {
            if rb[i] == 0 {
                continue;
            }
            maxi_b = maxi_b.min(f[i] / rb[i]);
        }
        ans = ans.max(maxi_b + pa);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
