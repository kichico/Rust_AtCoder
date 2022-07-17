#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::Roots;
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
#[allow(non_snake_case)]

fn factors(x: i64) -> i64 {
    let limit = (x as f32).sqrt() as i64 + 1;
    let mut ret = 0;
    for i in 1..limit {
        if x % i == 0 {
            ret += 2;
            if i * i == x {
                ret -= 1;
            }
        }
    }
    return ret;
}

fn solve() {
    input! {
        n: usize,
    }
    let mut current = 1;
    for i in 1..=n {
        factors(i as i64);
    }
    println!("");
}

fn main() {
    solve();
}
