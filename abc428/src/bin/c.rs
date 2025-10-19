use indexing::container_traits::Pushable;
use proconio::input;
const INF: usize = 1 << 30;

fn main() {
    input! {
        q: usize,
    }

    let mut left_remain_cnt = 0;
    let mut right_over_index = INF;
    let mut stack = vec![];
    for _ in 0..q {
        input! {
            op: usize,
        }
        match op {
            1 => {
                input! {
                    c: char,
                }
                if c == '(' {
                    left_remain_cnt += 1;
                } else {
                    left_remain_cnt -= 1;
                    if left_remain_cnt < 0 && right_over_index == INF {
                        right_over_index = stack.len();
                    }
                }
                stack.push(c);
                println!(
                    "{}",
                    if left_remain_cnt == 0 && right_over_index == INF {
                        "Yes"
                    } else {
                        "No"
                    }
                );
            }
            2 => {
                let c = stack.pop().unwrap();
                if stack.len() == right_over_index {
                    right_over_index = INF;
                }
                if c == '(' {
                    left_remain_cnt -= 1;
                } else {
                    left_remain_cnt += 1;
                }
                println!(
                    "{}",
                    if left_remain_cnt == 0 && right_over_index == INF {
                        "Yes"
                    } else {
                        "No"
                    }
                );
            }
            _ => unreachable!(),
        }
    }
}
