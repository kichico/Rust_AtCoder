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
        n: usize,a:[i64;n]
    }
    let mut base = vec![0; 4];
    let mut p = 0;
    for i in 0..n {
        base[0] += 1;
        let m = a[i];
        for j in (0..4).rev() {
            if j + m as usize >= 4 {
                p += base[j];
                base[j] = 0;
            } else {
                base[j + m as usize] = base[j];
                base[j] = 0;
            }
        }
    }
    println!("{}", p);
}

fn main() {
    solve();
}
