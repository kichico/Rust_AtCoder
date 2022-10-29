use itertools::enumerate;
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
        n:usize,mut K:usize,mut ringo:[usize;n]
    }
    let mut rest: HashSet<usize> = HashSet::new();
    let mut mini = usize::MAX;
    for (i, x) in enumerate(&ringo) {
        if *x != 0 {
            rest.insert(i);
            mini = min(mini, x.clone());
        }
    }
    let mut current = 0;
    loop {
        for i in 0..n {
            if ringo[i] > 0 {
                mini = min(mini, ringo[i]);
            }
        }
        if mini >= K {
            break;
        }
        current += mini * rest.len();
        K -= mini;
        for i in 0..n {
            if ringo[i] == 0 {
                continue;
            }
            ringo[i] -= mini;
            if ringo[i] == 0 {
                rest.remove(&i);
            }
        }
    }
    let mut pos = n + 1;
    for i in 0..n {
        if ringo[i] == mini {
            pos = i;
            break;
        }
    }
    println!("{}", mini);
    for i in 0..=pos {
        if ringo[i] >= mini {
            ringo[i] -= mini;
        }
    }
    for i in pos..n {
        if ringo[i] > mini {
            ringo[i] -= (mini - 1);
        }
    }
    println!("{}", ringo.iter().join(" "));
}

fn main() {
    solve();
}
