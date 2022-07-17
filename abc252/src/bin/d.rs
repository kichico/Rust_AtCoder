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
    let mut dict: HashMap<i64, i64> = HashMap::new();
    for i in 0..n {
        *dict.entry(a[i]).or_insert(0) += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        let num = n as i64 - dict.get(&a[i]).unwrap();
    }
}

fn main() {
    solve();
}
