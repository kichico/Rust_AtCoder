#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn solve() {
    input! {
        n: String,
    }
    let mut map: HashMap<char, i64> = HashMap::new();
    for i in n.as_str().chars() {
        let cnt = map.entry(i).or_insert(0);
        *cnt += 1;
    }
    let num: i64 = 8;
    while num + 8 < 1000 {
        let nums = num.to_string();
        let mut flg = true;
        let mut tm = HashMap::new();
        for x in nums.as_str().chars() {
            let c = tm.entry(x).or_insert(0 as i64);
            *c += 1;
        }
        for (k, v) in tm {
            if map[&k] < v {
                flg = false;
            }
        }
        if flg == true {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn main() {
    solve();
}
