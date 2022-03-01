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
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        m: usize,
        cake: [(i64,i64,i64);n],
    }
    let mut dp: Vec<Vec<(i64, i64, i64)>> = vec![vec![(0, 0, 0); n + 1]; m + 1];
    dp[0][0] = cake[0];
    for i in 1..n {
        dp[0][i] = cake[i];
        if dp[0][i - 1].0.abs() + dp[0][i - 1].1.abs() + dp[0][i - 1].2.abs()
            > cake[i].0.abs() + cake[i].1.abs() + cake[i].2.abs()
        {
            dp[0][i] = dp[0][i - 1];
        }
    }
    for j in 0..n - 1 {
        for i in 0..m {
            let left = dp[i][j].0.abs() + dp[i][j].1.abs() + dp[i][j].2.abs();
            let right = (dp[i][j].0 + cake[j + 1].0).abs()
                + (dp[i][j].1 + cake[j + 1].1).abs()
                + (dp[i][j].2 + cake[j + 1].2).abs();
            dbg!(&i, &j, &left, &right);
            if left < right {
                dp[i + 1][j + 1].0 = dp[i][j].0 + cake[j + 1].0;
                dp[i + 1][j + 1].1 = dp[i][j].1 + cake[j + 1].1;
                dp[i + 1][j + 1].2 = dp[i][j].2 + cake[j + 1].2;
            } else {
                dp[i][j + 1] = dp[i][j];
            }
        }
    }
    for i in 0..m + 1 {
        let mut v: String = String::new();
        for j in 0..n {
            v += &(dp[i][j].0.abs() + dp[i][j].1.abs() + dp[i][j].2.abs()).to_string();
            v += " ";
        }
        println!("{}", v);
    }
    println!(
        "{}",
        dp[m - 1][n - 1].0.abs() + dp[m - 1][n - 1].1.abs() + dp[m - 1][n - 1].2.abs()
    );
}

fn main() {
    solve();
}
