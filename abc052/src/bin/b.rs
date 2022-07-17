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
        mut n:i64,s:String
    }
    let mut ans = 0;
    let mut value = 0;
    let s: Vec<char> = s.chars().collect();
    for c in s {
        if c == 'I' {
            value += 1;
        } else {
            value -= 1;
        }
        ans = ans.max(value);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
