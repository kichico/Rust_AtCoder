#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        a: [i64;n],
        b: [i64;n],
    }
    let mut bite = 0;
    for i in 0..n {
        if a[i] == b[i] {
            bite += 1;
        }
    }
    let mut hit = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if a[i] == b[j] {
                hit += 1;
            }
        }
    }
    println!("{}\n{}", bite, hit);
}

fn main() {
    solve();
}
