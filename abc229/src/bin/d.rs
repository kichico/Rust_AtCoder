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
        s:Chars,k:i64
    }
    let mut ans = 0;
    let mut right = 0;
    let n = s.len();
    let mut dot = vec![0; n + 1];
    for i in 0..n {
        dot[i + 1] = dot[i].clone();
        if s[i] == '.' {
            dot[i + 1] += 1;
        }
    }
    for left in 0..s.len() {
        right = right.max(left);
        while right < n && dot[right + 1] - dot[left] <= k {
            right += 1;
        }
        ans = ans.max(right - left);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
