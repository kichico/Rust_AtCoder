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
        n: usize,a:i64,b:i64,x:[i64;n]
    }
    let mut ans = 0;
    for i in 1..n {
        let dist = x[i] - x[i - 1];
        ans += min(dist * a, b);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
