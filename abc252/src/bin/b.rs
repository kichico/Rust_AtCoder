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
use std::iter::FromIterator;
#[allow(non_snake_case)]
fn solve() {
    input! {
        n: usize,k:usize,mut a:[i64;n],b:[i64;k]
    }
    let kirai: HashSet<i64> = HashSet::from_iter(b.into_iter());
    let mut a = a.iter().enumerate().collect::<Vec<_>>();
    a.sort_by(|x, y| x.1.cmp(y.1));
    let maxi = a.iter().next_back().unwrap().1;
    let mut ans = "No";
    for i in 0..n {
        if a[i].1 == maxi && kirai.contains(&(a[i].0 as i64 + 1)) {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
