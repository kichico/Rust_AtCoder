#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        N: usize,
    }
    if N % 9 != 0 {
        println!("0");
        return;
    }
    let mut dp: Vec<i64> = vec![0; N + 1];
    let MOD = 1e9 as i64 + 7;
    dp[0] = 1;
    for i in 0..=N {
        let bd = min(i, 9);
        for j in 1..=bd {
            dp[i] += dp[i - j];
            dp[i] %= MOD;
        }
        dp[i] %= MOD;
    }
    println!("{}", dp[N] % MOD);
}

fn main() {
    solve();
}
