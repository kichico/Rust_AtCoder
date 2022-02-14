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
    let vec = vec![
        2, 20, 19, 10, 20, 3, 5, 7, 2, 4, 3, 11, 20, 15, 4, 7, 13, 19, 1, 8,
    ];
    for i in vec {
        println!("{}", i);
    }
}

fn main() {
    solve();
}
