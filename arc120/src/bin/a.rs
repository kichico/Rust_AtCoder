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
        a:[i64;n]
    }
    let mut sum = vec![0; n];
    let mut kthMax: BTreeSet<i64> = BTreeSet::new();
    sum[0] = a[0];
    kthMax.insert(a[0]);
    for i in 1..n {
        sum[i] = sum[i - 1] + a[i];
    }
    let mut kthSum = vec![0; n];
}

fn main() {
    solve();
}
