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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        mut V: i64,
        a: [i64;3],
    }
    loop {
        for i in 0..3 {
            if V - a[i] < 0 {
                let ans = if i == 0 {
                    "F"
                } else if i == 1 {
                    "M"
                } else {
                    "T"
                };
                println!("{}", ans);
                return;
            }
            V -= a[i];
        }
    }
}

fn main() {
    solve();
}
