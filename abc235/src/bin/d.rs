#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        a: i64,
        N: i64,
    }
    let mut que: VecDeque<i64> = VecDeque::new();
    let mut d: Vec<i64> = vec![-1; 10 * N as usize];
    d[1] = 0;
    que.push_back(1);
    while !que.is_empty() {
        let v = *que.front().unwrap();
        que.pop_front();
        let s: String = v.to_string();
        if v * a < 10 * N && d[(v * a) as usize] == -1 {
            d[(v * a) as usize] = d[v as usize] + 1;
            que.push_back(v * a);
        }
        if s.len() >= 2 && s.len() <= (N.clone()).to_string().len() {
            let mut c: VecDeque<char> = s.as_str().chars().collect();
            if c.back() == Some(&'0') {
                continue;
            }
            c.rotate_right(1);
            let nv: Vec<u8> = c.iter().map(|x| *x as u8 - 48).collect::<Vec<u8>>();
            let mut num = 0;
            for conv in 0..nv.len() {
                num += (nv[conv] as i64) * 10i64.pow((nv.len() - conv - 1) as u32);
            }
            if num >= 10 * N {
                continue;
            }
            if d[num as usize] != -1 {
                continue;
            } else {
                d[num as usize] = d[v as usize] + 1;
                que.push_back(num);
            }
        }
    }
    println!("{}", d[N as usize]);
}

fn main() {
    solve();
}
