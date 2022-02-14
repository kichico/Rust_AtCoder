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
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        mut a: i128,
        mut b: i128,
        mut k: i128,
    }
    let mut dict: BTreeMap<(i128, i128), i128> = BTreeMap::new();
    for i in (1i128..=60i128).rev() {
        let mut bunshi = i.clone();
        let mut bunbo: i128 = 1;
        for j in 1..=60 {
            if i < j {
                break;
            }
            dict.insert((i, j), bunshi / bunbo);
            bunshi *= i - j;
            if bunshi % (j + 1) == 0 {
                bunshi /= j + 1;
            } else {
                bunbo *= j + 1;
            }
        }
    }
    let len = a.clone() + b.clone();
    let mut s: String = String::new();
    for i in 0..len {
        if a == 0 {
            s.push('b');
            b -= 1;
        } else if b == 0 {
            s.push('a');
            a -= 1;
        } else {
            let ncr: i128 = dict[&(len - i - 1, b)];
            dbg!(&(len - i - 1), &b, &ncr, &k);
            if k > ncr {
                s.push('b');
                k -= ncr;
                b -= 1;
            } else {
                s.push('a');
                a -= 1;
            }
        }
    }
    println!("{}", s);
}

fn main() {
    solve();
}
