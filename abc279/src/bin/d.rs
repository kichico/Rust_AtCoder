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
        a:i128,b:i128
    }
    let mut left = 0;
    let mut right = 1e18 as i128 + 1;
    while right - left > 5 {
        let c1 = (left * 2 + right) / 3;
        let c2 = (left + right * 2) / 3;
        let time_left = (b * c1) as f64 + a as f64 / (1f64 + c1 as f64).sqrt();
        let time_right = (b * c2) as f64 + a as f64 / (1f64 + c2 as f64).sqrt();
        if time_left < time_right {
            right = c2;
        } else {
            left = c1;
        }
    }
    let mut ans = std::f64::MAX;
    let eps = 1e-7;
    for i in left..=right {
        let v = (b * i) as f64 + (a as f64 / (1f64 + i as f64).sqrt());
        if ans > v {
            ans = v;
        }
    }
    println!("{:.10}", ans);
}
fn main() {
    solve();
}
