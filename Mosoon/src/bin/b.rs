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
    input! {mut n:i64}
    while n > 1000 {
        n -= 1000;
    }
    println!("{}", 1000 - n);
}

fn main() {
    solve();
}
