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
#[fastout]
fn solve() {
    input! {
        mut n: i64
    }
    let mut hh = 21;
    let mut mm = 0;
    if n >= 60 {
        hh += 1;
        n -= 60;
    }
    mm += n;
    let mut mm: Vec<char> = mm.to_string().chars().rev().collect();
    while mm.len() < 2 {
        mm.push('0');
    }
    let mm: String = mm.iter().rev().collect();
    println!("{}:{}", hh, mm);
}

fn main() {
    solve();
}
