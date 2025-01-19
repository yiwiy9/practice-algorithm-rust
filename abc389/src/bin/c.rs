use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut deq = std::collections::VecDeque::new();
    let mut acc = 0;
    let mut sub = 0;
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    l: usize,
                }
                deq.push_back((acc, l));
                acc += l;
            }
            2 => {
                let (_, l) = deq.pop_front().unwrap();
                sub += l;
            }
            3 => {
                input! {
                    k: usize,
                }
                println!("{}", deq[k - 1].0 - sub);
            }
            _ => unreachable!(),
        }
    }
}
