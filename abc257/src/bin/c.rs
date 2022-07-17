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
        n: usize,s:String, w:[i64;n]
    }
    let weight: Vec<_> = w.iter().enumerate().collect();
    let mut dic: BTreeMap<i64, (usize, usize)> = BTreeMap::new();
    let s: Vec<char> = s.chars().collect();
    for (i, w) in weight {
        let e = dic.entry(*w).or_insert((0, 0));
        if s[i] == '0' {
            e.0 += 1;
        } else {
            e.1 += 1;
        }
    }
    let mut adult = 0;
    let mut child = 0;
    for i in 0..n {
        if s[i] == '1' {
            adult += 1;
        }
    }
    let mut ans = adult;
    for (_, (c, a)) in dic {
        child += c;
        adult -= a;
        ans = max(ans, child + adult);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
