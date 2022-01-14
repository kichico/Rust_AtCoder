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
        n: i64,
        k: i64,
    }
    let mut ans: i64 = 10000;
    for i in 0..=n {
        let mut current = 1;
        for _ in 0..i {
            current *= 2;
        }
        current += (n - i) * k;
        ans = min(current, ans);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
