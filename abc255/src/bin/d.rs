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
        n: usize,q:usize,mut a:[i64;n],query:[i64;q]
    }
    a.sort();
    let mut sum = vec![0; n];
    sum[0] = a[0];
    for i in 1..n {
        sum[i] = sum[i - 1] + a[i];
    }
    for i in 0..q {
        let mut left = 0;
        let mut right = n;
        let x = query[i];
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if a[mid] >= x {
                right = mid;
            } else {
                left = mid;
            }
            //println!("left:{} right:{}", left, right);
        }
        let lsum = (left + 1) as i64 * x;
        let rsum = (n - right) as i64 * x;
        if x <= a[left] {
            println!("{}", sum[n - 1] - x * n as i64);
        } else if x >= a[n - 1] {
            println!("{}", x * n as i64 - sum[n - 1]);
        } else {
            //println!("left:{} right:{} lsum:{} rsum:{}", left, right, lsum, rsum);
            //println!("sum[{}]:{}", right, sum[n - 1] - sum[left]);
            println!("{}", lsum - sum[left] + (sum[n - 1] - sum[left]) - rsum);
        }
    }
}

fn main() {
    solve();
}
