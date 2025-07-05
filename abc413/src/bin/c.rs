use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut deque = std::collections::VecDeque::new();
    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    c: usize,
                    x: usize,
                }
                deque.push_back((c, x));
            }
            2 => {
                input! {
                    k: usize,
                }
                let mut count = 0;
                let mut sum = 0;
                while let Some((c, x)) = deque.pop_front() {
                    count += c;
                    if count < k {
                        sum += x * c;
                        continue;
                    }

                    if count == k {
                        sum += x * c;
                        println!("{}", sum);
                    } else {
                        let remaining = count - k;
                        sum += x * (c - remaining);
                        deque.push_front((remaining, x));
                        println!("{}", sum);
                    }
                    break;
                }
            }
            _ => unreachable!(),
        }
    }
}
