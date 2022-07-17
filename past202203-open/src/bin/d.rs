#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::Roots;
#[allow(unused_imports)]
use petgraph::*;
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
#[fastout]
fn solve() {
    input! {
        t:usize,n: usize,score:[[i64;n];t]
    }
    let mut cMax = vec![0i64; n];
    for i in 0..t {
        for j in 0..n {
            cMax[j] = cMax[j].max(score[i][j]);
        }
        let ans: i64 = cMax.iter().sum();
        println!("{}", ans);
    }
}

fn main() {
    solve();
}
