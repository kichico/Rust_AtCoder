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
        n: usize,
        a: [i64;n],
    }
    let mut sum: i64 = 0;
    for x in &a {
        sum += x;
    }
    let mut ss: Vec<i64> = vec![0i64; 2 * n];
    ss[0] = a[0];
    for i in 1..n {
        ss[i] = a[i] + ss[i - 1];
    }
    for i in 0..n {
        ss[i + n] = a[i] + ss[i + n - 1];
    }
    for i in 0..n {
        let mut left = i;
        let mut right = 2 * n;
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            let cake = ss[mid] - ss[i];
            //dbg!(&left, &right, &cake);
            if cake * 10 > sum {
                right = mid;
            } else {
                left = mid;
            }
        }
        if (ss[left] - ss[i]) * 10 == sum {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn main() {
    solve();
}
