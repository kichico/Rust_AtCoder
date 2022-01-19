#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
fn modpow(mut x: i128, mut n: i128, MOD: i128) -> i128 {
    let mut ans = 1;
    while n > 0 {
        if n & 1 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n >>= 1;
    }
    return ans;
}

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: i128,
        k: i128,
    }
    let MOD = 1e9 as i128 + 7;
    let ans = if n == 1 {
        k
    } else if n == 2 {
        k * (k - 1)
    } else {
        k * (k - 1) * modpow(k - 2, n - 2, MOD)
    };
    println!("{}", ans % MOD);
}

fn main() {
    solve();
}
