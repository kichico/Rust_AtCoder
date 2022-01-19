#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
#[allow(non_snake_case)]
fn EtoT(s: Vec<char>) -> i128 {
    let mut retv: i128 = 0;
    for x in (0..s.len()).rev() {
        retv += (s[s.len() - x - 1] as i128 - '0' as i128) * 8i128.pow(x as u32);
    }
    return retv;
}

#[allow(non_snake_case)]
fn TtoN(mut x: i128) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    if x == 0 {
        v.push('0');
    }
    while x > 0 {
        let c = std::char::from_digit((x % 9) as u32, 10).unwrap();
        v.push(c);
        x /= 9;
    }
    v.reverse();
    return v;
}
#[fastout]
#[allow(non_snake_case)]
fn solve() {
    input! {
        N: i128,
        K: i128,
    }
    let mut current: Vec<char> = N.to_string().chars().collect();
    for _ in 0..K {
        let num = EtoT(current);
        let mut nine = TtoN(num);
        for x in 0..nine.len() {
            if nine[x] == '8' {
                nine[x] = '5';
            }
        }
        current = nine;
    }
    let ans: String = current.iter().collect();
    println!("{}", &ans);
}

fn main() {
    solve();
}
