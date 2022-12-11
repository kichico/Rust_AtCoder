#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::integer::{gcd, lcm};
use num_integer::Roots;
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
fn solve() {
    input! {
        n:i64
    }
    let mut square: Vec<i64> = Vec::new();
    for i in 1..=n {
        square.push(i * i);
    }
    let mut ans = 0;
    println!("comp {}", square.len());
    for i in square {
        for v in 1..=i.sqrt() {
            if i % v == 0 && i / v <= n && v <= n {
                let div = i / v;
                if div == v {
                    ans += 1;
                } else {
                    ans += 2;
                }
            }
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
