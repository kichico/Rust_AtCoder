use alga::general::Field;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use num_integer::sqrt;
#[allow(unused_imports)]
use num_integer::Roots;
#[allow(unused_imports)]
use petgraph::unionfind;
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
#[allow(non_snake_case)]

fn dist(x: (i64, i64), y: (i64, i64)) -> f64 {
    return 
}
fn solve() {
    input! {
       n:usize,p:[(i64,i64);n]
    }
    let mut ans = 0.0;
    for i in 0..n {
        for j in 0..n {
            ans = ans.max(dist(p[i], p[j]));
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
