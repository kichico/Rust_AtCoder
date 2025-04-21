#[allow(unused_imports)]
use itertools::*;
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use rand::Rng;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::hash::Hash;
#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn random() {
    for _ in 0..100 {
        let mut rand = rand::thread_rng();
        let n: usize = rand.gen_range(0..10);
        let mut x = (0..n)
            .into_iter()
            .map(|x| rand.gen_range(-1000i64..1000))
            .sorted()
            .collect::<Vec<i64>>();
        let population = (0..n)
            .into_iter()
            .map(|x| rand.gen_range(0i64..1000))
            .sorted()
            .collect::<Vec<i64>>();
        let q: usize = rand.gen_range(0..10);
        let query: Vec<(i64, i64)> = (0..q)
            .into_iter()
            .map(|x| {
                (
                    rand.gen_range(-1000..1000i64),
                    rand.gen_range(-1000..1000i64),
                )
            })
            .map(|(x, y)| (x.min(y), x.max(y)))
            .collect::<Vec<_>>();
        x.push(i64::MAX);
        let cumsum = population.iter().cumsum::<i64>().collect::<Vec<_>>();
        let mut ans = Vec::new();
        let mut guchoku = Vec::new();
        for (mut l, mut r) in query {
            if l > r {
                let t = r.clone();
                r = l;
                l = t;
            }
            let mut gu = 0;
            for index in l..=r {
                let p = x.lower_bound(&index);
                if x[p] == index {
                    gu += population[p];
                }
            }
            guchoku.push(gu);
            if r < x[0] {
                ans.push(0);
                continue;
            }
            let left = x.lower_bound(&l);
            let right = x.upper_bound(&r) - 1;
            //println!("{} {}", left, right);
            if right < left || r < x[right] {
                ans.push(0i64);
                continue;
            }
            if right == left && l <= x[left] && x[right] <= r {
                ans.push(x[left]);
                continue;
            }
            if left > 0 {
                ans.push(cumsum[right] - cumsum[left - 1]);
            } else {
                ans.push(cumsum[right]);
            }
        }
        for (ac, gu) in ans.iter().zip(guchoku) {
            assert!(ac == &gu);
        }
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut x:[i64;n],mut population:[i64;n],q:usize,query:[(i64,i64);q]
    }
    x.push(i64::MAX);
    let cumsum = population.iter().cumsum::<i64>().collect::<Vec<_>>();
    let mut ans = Vec::new();
    for (l, r) in query {
        if r < x[0] {
            ans.push(0i64);
            continue;
        }
        let left = x.lower_bound(&l);
        let right = x.upper_bound(&r) - 1;
        //println!("{} {}", left, right);
        if right < left || r < x[right] {
            ans.push(0i64);
            continue;
        }
        if right == left && l <= x[left] && x[right] <= r {
            ans.push(x[left]);
            continue;
        }
        if left > 0 {
            ans.push(cumsum[right] - cumsum[left - 1]);
        } else {
            ans.push(cumsum[right]);
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
