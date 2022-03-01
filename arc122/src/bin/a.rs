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
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(non_snake_case)]
fn open_file(x: String, mut a: Vec<i64>, mut n: usize) -> (Vec<i64>, usize) {
    println!("{}", x);
    let file = File::open(x).unwrap();
    let buf = BufReader::new(file);
    let mut cnt = 0;
    for line in buf.lines() {
        let li = line.unwrap();
        let p: Vec<&str> = li.as_str().split_whitespace().collect();
        if cnt == 0 {
            let mut ss: Vec<char> = p[0].chars().collect();
            ss.reverse();
            while ss.last().unwrap() != &'3' {
                ss.pop();
            }
            ss.reverse();
            let pp: String = ss.iter().collect();
            n = pp.parse().unwrap();
        } else {
            for v in p {
                a.push(v.parse().unwrap());
            }
        }
        cnt += 1;
    }
    return (a, n);
}

fn solve() {
    let x: String = "C:/Users/sueki/source/repos/Rust_AtCoder/arc122/src/bin/in.dat".to_string();
    let mut a: Vec<i64> = Vec::new();
    let mut n = 0;
    let ans = open_file(x, a, n);
    let a = ans.0;
    let n = ans.1;
    let mut dp = vec![vec![(0i64, 0i64); 2]; n + 1]; //(value,cnt)
    if n == 1 {
        println!("{}", a[0]);
        return;
    }
    dp[0][0] = (a[0], 1);
    dp[0][1] = (a[0], 1);
    dp[1][0] = (dp[0][0].0 + a[1], 1);
    dp[1][1] = (dp[0][0].0 - a[1], 1);
    let MOD = 1e9 as i64 + 7 as i64;
    for i in 2..n {
        //dbg!(&(dp[i - 1][0].0 + dp[i - 1][1].0));
        dp[i][0] = (
            (dp[i - 1][0].0 + ((dp[i - 1][0].1 + dp[i - 1][1].1) * (a[i] % MOD)) + dp[i - 1][1].0),
            dp[i - 1][0].1 + dp[i - 1][1].1,
        );
        dp[i][1] = (
            (dp[i - 1][0].0 - (dp[i - 1][0].1 * (a[i] % MOD))) % MOD,
            dp[i - 1][0].1,
        );
        dp[i][0].0 %= MOD;
        dp[i][1].0 %= MOD;
        dbg!(i);
        //dbg!(&dp[i][0].0, &dp[i][1].0, &dp[i][0].1, &dp[i][1].1);
    }
    println!("{}", (dp[n - 1][0].0 + dp[n - 1][1].0) % MOD);
}

fn main() {
    solve();
}
