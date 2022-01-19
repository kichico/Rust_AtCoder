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
        s: String,
        t: String,
    }
    let mut st: BTreeSet<String> = BTreeSet::new();
    let N = s.len();
    let M = t.len();
    for R in (M..=N).rev() {
        //dbg!(M, R);
        let L = R - M;
        //dbg!(L);
        let mut flg = true;
        for i in 0..M {
            if s.chars().nth(L + i).unwrap() != '?'
                && s.chars().nth(L + i).unwrap() != t.chars().nth(i).unwrap()
            {
                flg = false;
            }
        }
        if flg == true {
            let mut ss: Vec<char> = s.clone().chars().collect();
            for i in 0..M {
                ss[L + i] = t.chars().nth(i).unwrap();
            }
            for i in 0..ss.len() {
                if ss[i] == '?' {
                    ss[i] = 'a';
                }
            }
            st.insert(ss.iter().collect());
        }
    }
    if st.len() == 0 {
        println!("UNRESTORABLE");
    } else {
        let min = st.iter().next().unwrap();
        println!("{}", min);
    }
}

fn main() {
    solve();
}
