#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind;
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
        s: String,
    }
    let v: Vec<char> = s.as_str().chars().collect();
    let mut flg = true;
    for c in 0..s.len() {
        if c % 2 == 0 && v[c].is_uppercase() {
            flg = false;
        }
        if c % 2 != 0 && v[c].is_lowercase() {
            flg = false;
        }
        dbg!(flg);
    }
    println!("{}", if flg { "Yes" } else { "No" });
}

fn main() {
    solve();
}
