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
        n:usize,a:[i64;n]
    }
    let mut cumsum = vec![0; n];
    cumsum[0] = a[0];
    for i in 1..n {
        cumsum[i] = a[i] + cumsum[i - 1];
    }
    let mut cumsum_right = vec![0; n];
    cumsum_right[n - 1] = a[n - 1];
    for i in (0..n - 1).rev() {
        cumsum_right[i] = a[i] + cumsum_right[i + 1];
    }
    let mut ans = 1e18 as i64;
    for i in 0..n - 1 {
        let dist = abs(cumsum[i] - cumsum_right[i + 1]);
        ans = min(ans, dist);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
