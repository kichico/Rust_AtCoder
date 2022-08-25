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
use rand::*;
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
    let mut gen = rand::thread_rng();
    for i in 0..30 {
        let n = gen.gen_range(10, 20);
        let mut a = vec![0; n];
        for j in 0..n {
            a[i] = gen.gen_range(10, 2000);
        }
        // naive
        for j in 0..n - 2 {
            for k in j..n - 1 {
                for m in k..n {
                    let (x, y, z) = (a[j], a[k], a[m]);
                    let mut ans: BTreeSet<i64> = BTreeSet::new();
                    let mut v = vec![x, y, z];
                    v.sort();
                }
            }
        }
    }
}

fn main() {
    solve();
}
