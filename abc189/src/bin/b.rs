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
        n: usize,mut x:i64,a:[(i64,i64);n]
    }
    let mut current = 0;
    x *= 100;
    for i in 0..n {
        let (v, per) = a[i];
        current += v * per;
        if current > x {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}

fn main() {
    solve();
}
