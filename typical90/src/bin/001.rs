#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,length:usize,k:usize,a:[i64;n]
    }
    let (mut left, mut right): (usize, usize) = (0, length + 1);
    while right - left > 1 {
        let mid: usize = left + (right - left) / 2;
        let mut current = 0;
        let mut cnt = k.clone();
        let mut mini = length as i64;
        let mut i = 0;
        while cnt > 0 && i < n {
            if a[i] - current >= mid as i64 {
                mini = mini.min(a[i] - current);
                current = a[i];
                cnt -= 1;
            }
            i += 1;
        }
        mini = mini.min(length as i64 - current);
        if mini < mid as i64 || cnt != 0 {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", left);
}

fn main() {
    solve();
}
