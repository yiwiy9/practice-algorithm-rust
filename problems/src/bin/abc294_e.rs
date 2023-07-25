use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        _: usize,
        n1: usize,
        n2: usize,
        vl1: [(usize,usize); n1],
        vl2: [(usize,usize); n2],
    }

    let mut vl_deq1 = VecDeque::from(vl1);
    let mut vl_deq2 = VecDeque::from(vl2);

    let mut ans: usize = 0;
    let mut v1 = 0;
    let mut l1 = 0;
    let mut v2 = 0;
    let mut l2 = 0;
    loop {
        if vl_deq1.is_empty() && vl_deq2.is_empty() {
            break;
        }

        if l1 == 0 {
            let front_vl = vl_deq1.pop_front().unwrap();
            v1 = front_vl.0;
            l1 = front_vl.1;
        }
        if l2 == 0 {
            let front_vl = vl_deq2.pop_front().unwrap();
            v2 = front_vl.0;
            l2 = front_vl.1;
        }

        if l1 < l2 {
            if v1 == v2 {
                ans += l1
            }
            l2 -= l1;
            l1 -= l1;
        } else {
            if v1 == v2 {
                ans += l2
            }
            l1 -= l2;
            l2 -= l2;
        }
    }

    println!("{}", ans);
}
