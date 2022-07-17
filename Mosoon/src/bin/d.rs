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
        n:usize,m:usize,query:[i64;m],
    }
    let mut order: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut rec: BTreeMap<i64, i64> = BTreeMap::new();
    for i in 1..n as i64 + 1 {
        order.insert((i, i));
        rec.insert(i, i);
    }
    for i in 0..m {
        let top = query[i];
        let ord = *rec.get(&top).unwrap();
        order.remove(&(ord, top));
        let mini = *order.iter().next().unwrap();
        let ord = mini.0;
        order.insert((ord - 1, top));
        let x = rec.get_mut(&top).unwrap();
        *x = ord - 1;
    }
    for i in order {
        println!("{}", i.1);
    }
}

fn main() {
    solve();
}
