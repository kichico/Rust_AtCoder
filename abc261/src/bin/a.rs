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
#[fastout]
fn solve() {
    input! {
        a:i64,b:i64,c:i64,d:i64
    }
    if b <= c || d <= a {
        println!("0");
        return;
    }
    let ans = if c < a {
        if b <= d {
            b - a
        } else {
            d - a
        }
    } else if c < b {
        if b <= d {
            b - c
        } else {
            d - c
        }
    } else {
        0
    };
    println!("{}", ans);
}

fn main() {
    solve();
}
