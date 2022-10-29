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
        n:usize,mut w:i64,mut cheese:[(i64,i64);n]
    }
    cheese.sort_by(|x, y| x.0.cmp(&y.0).reverse());
    let mut ans = 0;
    for i in 0..n {
        let (a, b) = cheese[i];
        if b > w {
            ans += a * w;
            break;
        } else {
            ans += a * b;
            w -= b;
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
