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
        Ma: usize,
        Mb: usize,
        med: [(i64,i64,i64);n],
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![i64::MAX; 410]; 410];
}

fn main() {
    solve();
}
