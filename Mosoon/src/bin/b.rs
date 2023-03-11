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
    input! {
        n:usize,mut len:[(Usize1,Usize1);n]
    }
    let mut isu = vec![false; 1e5 as usize];
    for (l, r) in len {
        for i in l..=r {
            isu[i] = true;
        }
    }
    let mut cnt = 0;
    for i in 0..1e5 as usize {
        if isu[i] {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}

fn main() {
    solve();
}
