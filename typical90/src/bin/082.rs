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
        L:i128,
        R:i128,
    }
    let MOD = 1e9 as i128 + 7;
    let mut ans = 0;
    let keta = L.to_string().len() as i128;
    let mut now = L;
    let limit = R.to_string().len() as i128;
    for k in keta..=limit {
        let limit = min(R, 10i128.pow(k as u32) - 1);
        let num = (limit - now + 1) * (limit + now) * k / 2;
        ans += num;
        ans %= MOD;
        now = 10i128.pow((k) as u32);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
