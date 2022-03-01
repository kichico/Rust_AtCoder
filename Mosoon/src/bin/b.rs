#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
#[allow(non_snake_case)]
#[fastout]

fn solve() {
    input! {
        a: i64,
        op: char,
        b: i64,
    }
    let res = if op == '+' { a + b } else { a - b };
    println!("{}", res);
}

fn main() {
    solve();
}
