#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn solve() {
    input! {
        n: i64,
    }
    let ans: &str = if n % 2 == 0 { "White" } else { "Black" };
    println!("{}",ans);
}

fn main() {
    solve();
}
