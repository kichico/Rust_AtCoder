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

fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        mut n: i64,
    }
    let m = 2 * n;
    let mut ans: Vec<char> = Vec::new();
    for div in (1..5).rev() {
        while n - div >= 0 {
            n -= div;
            ans.push(to_char(div));
        }
    }
    ans.reverse();
    let ans: String = ans.iter().collect();
    println!("{}\n{}", m, ans);
}

fn main() {
    solve();
}
