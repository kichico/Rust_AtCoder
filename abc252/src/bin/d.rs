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
    let mut ans = 0;
    let mut total = n.clone() as i64;
    let mut some_num = 0;
    for (x, num) in &dict {
        some_num += num * (num - 1) / 2;
    }
    for (_x, num) in &dict {
        some_num = 0.max(some_num - num * (num - 1) / 2);
        total -= num;
        ans += (*num) * ((total * (total - 1)) / 2 - some_num);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
