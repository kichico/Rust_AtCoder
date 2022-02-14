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
        n: u64,
    }
    let keta = n.to_string().len() as u32;
    let mut previous: i128 = 0;
    let mut ans = 0;
    let MOD = 998244353;
    for i in 1..=keta {
        let upper = min(10i128.pow(i) - 1, n as i128);
        let lower = previous + 1;
        let value = ((1 + upper - previous) * (upper - lower + 1) / 2) as i128;
        ans += value;
        ans %= MOD;
        previous = upper;
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
