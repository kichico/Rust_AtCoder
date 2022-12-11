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
        mut a:[i64;3]
    }
    a.sort();
    let mut ans = 0;
    for i in 0..2 {
        while a[i] + 2 <= a[2] {
            a[i] += 2;
            ans += 1;
        }
    }
    if a[0] == a[1] && a[1] == a[2] {
        println!("{}", ans);
        return;
    }
    a.sort();
    if a[0] == a[1] {
        println!("{}", ans + 1);
    } else {
        println!("{}", ans + 2);
    }
}
fn main() {
    solve();
}
