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
        n: i128,
    }
    let mut ans: Vec<i128> = vec![1];
    let upper = 170141183460469231731687303715884105727 / 2;
    let mut now = 1;
    while now * 2 <= upper {
        now *= 2;
        ans.push(now);
        if ans.len() == 300 {
            break;
        }
    }
    println!("{}", ans.len());
    for i in ans {
        print!("{} ", i);
    }
}
fn main() {
    solve();
}
