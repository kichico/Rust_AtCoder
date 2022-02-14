#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        q: usize,
        a: [i64;n],
        query: [(i64,usize);q],
    }
    let mut dict: HashMap<i64, Vec<i64>> = HashMap::new();
    for i in 0..n {
        let k = dict.entry(a[i]).or_insert(Vec::<i64>::new());
        k.push(i as i64);
    }
    for (x, p) in query {
        if dict.contains_key(&x) == false {
            println!("-1");
            continue;
        }
        //let v = dict.get(&x);
        let v = dict.get(&x).unwrap();
        if v.len() < p {
            println!("-1");
        } else {
            println!("{}", dict[&x][p - 1] + 1);
        }
    }
}

fn main() {
    solve();
}
