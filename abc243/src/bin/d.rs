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
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
#[allow(non_snake_case)]
fn to_Binary(mut x: i128) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    if x == 0 {
        v.push('0');
    }
    while x > 0 {
        let c = std::char::from_digit((x % 2) as u32, 10).unwrap();
        v.push(c);
        x /= 2;
    }
    v.reverse();
    return v;
}
#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        mut p: i128,
        ss: String,
    }
    let s: Vec<char> = ss.chars().collect();
    let mut ans: Vec<char> = to_Binary(p);
    for i in 0..n {
        if s[i] == 'U' {
            ans.pop();
        } else if s[i] == 'L' {
            ans.push('0');
        } else {
            ans.push('1');
        }
    }
    let mut num: i128 = 0;
    for i in 0..ans.len() {
        if ans[i] == '1' {
            num += 2i128.pow((ans.len() - i - 1) as u32);
        }
        //dbg!(2i128.pow((ans.len() - i - 1) as u32));
    }
    //dbg!(&ans);
    println!("{}", num);
}

fn main() {
    solve();
}
