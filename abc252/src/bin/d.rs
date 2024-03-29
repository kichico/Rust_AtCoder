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
fn solve() {
    input! {
        n: usize,mut a:[i64;n]
    }
    a.sort();
    let mut dict: BTreeMap<i64, i64> = BTreeMap::new();
    for i in 0..n {
        *dict.entry(a[i]).or_insert(0) += 1;
    }
    let mut rest = dict.len() as i64;
    let mut ans = 0;
    for (x, num) in &dict {
        if rest <= 2 {
            continue;
        }
        ans += num * ((rest - 2) * (rest - 1)) / 2;
        rest -= 1;
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
