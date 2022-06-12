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
        weight: usize,
        n: usize,
        limit: usize,
        a: [(i64,i64);n],
    }
    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; limit + 1]; weight + 1]; n + 1];
    for j in 1..=limit {
        for w in 0..=weight as i64 {
            for i in 1..=n {
                dp[i][w as usize][j] = dp[i - 1][w as usize][j];
                if w - a[i - 1].0 >= 0 {
                    dp[i][w as usize][j] = max(
                        dp[i][w as usize][j],
                        dp[i - 1][(w - a[i - 1].0) as usize][j - 1] + a[i - 1].1,
                    );
                }
                // dbg!(&dp[i][w as usize][j]);
            }
        }
    }

    let mut ans = 0;
    for i in 0..=weight {
        ans = max(dp[n][i][limit], ans);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
