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
    }
    let mut dp: Vec<i64> = vec![0; n];
    dp[0] = 0;
    dp[1] = (a[1] - a[0]).abs();
    for i in 2..n {
        dp[i] = min(
            dp[i - 2] + (a[i] - a[i - 2]).abs(),
            dp[i - 1] + (a[i] - a[i - 1]).abs(),
        );
    }
    println!("{}", dp[n - 1]);
}

fn main() {
    solve();
}
