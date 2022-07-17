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
    input! {a:char,b:char}
    let a: i32 = a as i32;
    let b: i32 = b as i32;
    let ans = if a > b {
        ">"
    } else if a < b {
        "<"
    } else {
        "="
    };
    println!("{}", ans);
}

fn main() {
    solve();
}
