#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
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
fn local() {
    input! {
        k:i64
    }
    let mut ans = 0;
    for a in 1..=k {
        for b in 1..=k / a {
            let c = k / (a * b);
            if a * b * c <= k {
                ans += c;
            }
        }
    }
    println!("local {}", ans);
}

fn solve() {
    input! {
        k:usize
    }
    let mut ans = 0;
    for a in 1..=k {
        let bc = k / a;
        for b in 1..=bc {
            let c = k / (a * b);
            ans += c;
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
    //local();
}
