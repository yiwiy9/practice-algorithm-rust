use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort();

    let mut ans = vec![0; n + 1];
    let mut logout_days = BinaryHeap::new();
    let mut login_cnt = 0;
    let mut before_event_day = 0;
    for &(a, b) in &ab {
        while let Some(&Reverse(logout_day)) = logout_days.peek() {
            if logout_day > a {
                break;
            }
            ans[login_cnt] += logout_day - before_event_day;
            before_event_day = logout_day;
            logout_days.pop();
            login_cnt -= 1;
        }
        ans[login_cnt] += a - before_event_day;
        before_event_day = a;
        logout_days.push(Reverse(a + b));
        login_cnt += 1;
    }
    while let Some(&Reverse(logout_day)) = logout_days.peek() {
        ans[login_cnt] += logout_day - before_event_day;
        before_event_day = logout_day;
        logout_days.pop();
        login_cnt -= 1;
    }

    println!("{}", ans.iter().skip(1).join(" "));
}
