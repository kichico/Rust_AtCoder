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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: i128,
    }
    let mut ans: BTreeSet<i128> = BTreeSet::new();
    for b in 0..60u32 {
        let mut left = 0;
        let mut right = n + 1;
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if mid * 2i128.pow(b) > n {
                right = mid;
            } else {
                left = mid;
            }
        }
        let a = left;
        let c = n - (a * 2i128.pow(b));
        ans.insert(a + b as i128 + c);
    }
    println!("{}", ans.iter().next().unwrap());
}

fn main() {
    solve();
}
