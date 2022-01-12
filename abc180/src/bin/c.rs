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
    let lim = (n as f64).sqrt();
    let mut ans: BTreeSet<i64> = BTreeSet::new();
    for i in 1..=lim as i64 {
        if n % i == 0 {
            ans.insert(i);
            ans.insert(n / i);
        }
    }
    for x in ans {
        println!("{}", x);
    }
}

fn main() {
    solve();
}
