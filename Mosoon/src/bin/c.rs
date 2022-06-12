#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::integer::{gcd, lcm};
#[allow(unused_imports)]
use num_traits::abs;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    mem::swap,
};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n:usize,mut coupon:i64,x:i64,mut a:[i64;n]
    }
    a.sort_by(|x, y| x.cmp(y).reverse());
    for idx in 0..n {
        if a[idx] / x <= coupon {
            coupon -= a[idx] / x;
            a[idx] -= (a[idx] / x) * x;
        } else {
            a[idx] -= x * coupon;
            coupon = 0;
            break;
        }
    }

    a.sort_by(|x, y| x.cmp(y).reverse());
    if coupon != 0 {
        for i in 0..n {
            a[i] = 0;
            coupon -= 1;
            if coupon == 0 {
                break;
            }
        }
    }

    let sum: i64 = a.iter().sum();
    println!("{}", sum);
}

fn main() {
    solve();
}
