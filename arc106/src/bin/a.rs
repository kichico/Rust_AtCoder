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
        N: u64,
    }
    let mut cnt: u64 = 0;
    let mut v: u64 = 1;
    if N < 8 {
        println!("-1");
        return;
    }
    while v * 5 <= N {
        v *= 5;
        cnt += 1;
    }
    for B in 1..=cnt {
        let se = 5u64.pow(B as u32);
        if se > N {
            continue;
        }
        let mut fr = N - se;
        let mut A = 0;
        while fr % 3 == 0 && fr > 1 {
            fr /= 3;
            A += 1;
        }
        if fr == 1 && A > 0 {
            println!("{} {}", A, B);
            return;
        }
    }
    println!("-1");
}

fn main() {
    solve();
}
