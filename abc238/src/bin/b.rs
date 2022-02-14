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
        a: [i64;n],
    }
    let mut arg: BTreeSet<i64> = BTreeSet::new();
    arg.insert(0);
    for x in a {
        let mut next: BTreeSet<i64> = BTreeSet::new();
        for p in arg {
            next.insert((p + x) % 360);
        }
        arg = next;
        arg.insert(0);
    }
    let mut ans = 360 - arg.iter().last().unwrap();
    let vec: Vec<i64> = arg.into_iter().collect();
    for i in 0..n - 1 {
        ans = max(vec[i + 1] - vec[i], ans);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
