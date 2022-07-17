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
        n: String,
    }
    let mut idx = 0;
    let mut n: Vec<char> = n.chars().collect();
    while n.len() > 3 {
        for _i in 0..3 {
            n.pop();
        }
        idx += 1;
    }
    let s: String = n.iter().collect();
    println!("{}{}", s, ('a' as u8 + idx - 1) as char);
}

fn main() {
    solve();
}
