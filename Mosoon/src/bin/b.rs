#[allow(unused_imports)]
use itertools::Itertools;
#[allow(non_snake_case)]
#[allow(unused_imports)]
use num::{abs, integer::Roots};
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
fn solve() {
    input! { n:usize,k:usize,mut h:[i64;n] }
    h.sort();
    for i in 0..k {
        h.pop();
    }
    let ans: i64 = h.iter().sum();
    println!("{}", ans);
}

fn main() {
    solve();
}
