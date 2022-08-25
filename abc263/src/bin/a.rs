#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
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
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        a:[i64;5]
    }
    let mut dict: HashMap<i64, i64> = HashMap::new();
    for x in a {
        *dict.entry(x).or_insert(0) += 1;
    }
    for i in 1..14 {
        for j in 1..14 {
            let fr = dict.get(&i);
            let se = dict.get(&j);
            match fr {
                Some(x) => match se {
                    Some(y) => {
                        if (*x == 2 && *y == 3) || (*x == 3 && *y == 2) {
                            println!("Yes");
                            return;
                        }
                    }
                    None => continue,
                },
                None => continue,
            }
        }
    }
    println!("No");
}

fn main() {
    solve();
}
