#[allow(unused_imports)]
use itertools::Itertools;
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
        m: usize,
        edge: [(Usize1,Usize1);m],
    }
    let mut g: Vec<usize> = vec![0; n];
    for &(u, v) in &edge {
        g[u.max(v)] += 1;
    }
    let ans = g.iter().filter(|c| c == &&1).count();
    println!("{}", ans);
}

fn main() {
    solve();
}
