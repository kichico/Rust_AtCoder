#[allow(unused_imports)]
use itertools::Itertools;
use num_traits::Pow;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn solve() {
    input! {
        n: usize,
        a: [i64;n],
    }
    let mut man: i64 = 0;
    let mut euc: f64 = 0.0;
    let chevi = *a.iter().max().unwrap();
    for x in a {
        euc += (x as f64).pow(2);
        man += x.abs();
    }
    println!("{}\n{}\n{}\n", man, euc.sqrt(), chevi);
}

fn main() {
    solve();
}
