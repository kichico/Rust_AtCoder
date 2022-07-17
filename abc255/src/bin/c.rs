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
use std::mem::swap;
#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        X:i64,a:i64,mut d:i64,N:i64
    }
    let offset = a + d * (-1);
    let X = X - offset;
    let mut a = a - offset;
    let mut l = a + (N - 1) * d;
    if a > l {
        swap(&mut a, &mut l);
        d = d * -1;
    }
    if X <= a {
        println!("{}", abs(a - X));
    } else if X >= l {
        println!("{}", abs(l - X));
    } else {
        let mut left = 0;
        let mut right = N.clone();
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if a + (mid - 1) * d < X {
                left = mid;
            } else {
                right = mid;
            }
        }
        let left = a + (left - 1) * d;
        let right = a + (right - 1) * d;
        dbg!(&left, &right);
        let ans = min(abs(X - left), abs(X - right));
        println!("{}", ans);
    }
}

fn main() {
    solve();
}
