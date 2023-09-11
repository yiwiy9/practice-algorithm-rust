use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        mut k: usize,
    }

    if k <= 9 {
        println!("{}", k);
        return;
    }

    k -= 9;
    let mut deque = VecDeque::from((1..=9).collect::<Vec<usize>>());
    loop {
        let mut num = deque.pop_front().unwrap();
        let first_digit = num % 10;
        num *= 10;

        if first_digit > 0 {
            let next_num = num + first_digit - 1;
            deque.push_back(next_num);
            k -= 1;
            if k == 0 {
                println!("{}", next_num);
                return;
            }
        }

        let next_num = num + first_digit;
        deque.push_back(next_num);
        k -= 1;
        if k == 0 {
            println!("{}", next_num);
            return;
        }

        if first_digit < 9 {
            let next_num = num + first_digit + 1;
            deque.push_back(next_num);
            k -= 1;
            if k == 0 {
                println!("{}", next_num);
                return;
            }
        }
    }
}
