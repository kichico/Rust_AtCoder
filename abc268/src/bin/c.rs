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
        n:usize,a:[i64;n]
    }
    let mut dict: HashMap<i64, i64> = HashMap::new();
    for i in 0..n {
        let mut dist = i as i64 - a[i];
        if dist > (n as i64 / 2) {
            dist -= n as i64;
        }
        if dist < (n as i64 / 2) * -1 {
            dist += n as i64;
        }
        *dict.entry(dist).or_insert(0) += 1;
        let mut right = dist + 1;
        if right > (n as i64 / 2) {
            right -= n as i64;
        }
        *dict.entry(right).or_insert(0) += 1;
        let mut left = dist - 1;
        if left < (n as i64 / 2) * -1 {
            left += n as i64;
        }
        *dict.entry(left).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (_, v) in dict {
        ans = max(ans, v);
        //println!("k : {} v : {}", k, v);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
