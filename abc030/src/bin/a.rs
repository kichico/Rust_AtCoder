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
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let ans = if b * c < a * d {
        "AOKI"
    } else if b * c > a * d {
        "TAKAHASHI"
    } else {
        "DRAW"
    };
    println!("{}", ans);
}

fn main() {
    solve();
}
