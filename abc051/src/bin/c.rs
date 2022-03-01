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
        sx: i64,
        sy: i64,
        gx: i64,
        gy: i64,
    }
    let mut dirx: HashMap<i64, char> = HashMap::new();
    let mut diry: HashMap<i64, char> = HashMap::new();
}

fn main() {
    solve();
}
