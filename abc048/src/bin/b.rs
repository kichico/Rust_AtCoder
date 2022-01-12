use proconio::input;
fn div(n: i64, x: &i64) -> i64 {
    return if n < 0 { 0 } else { n / x + 1 };
}

fn solve() {
    input! {
        from: i64,
        to: i64,
        x: i64,
    }
    println!("{}", div(to, &x) - div(from - 1, &x));
}

fn main() {
    solve();
}
