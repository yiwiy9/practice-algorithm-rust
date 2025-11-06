use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut a = vec![];
    for _ in 0..q {
        input! {
            op: u8,
        }

        match op {
            1 => {
                input! {
                    x: usize,
                }
                a.push(x);
            }
            2 => {
                input! {
                    k: usize,
                }
                println!("{}", a[a.len() - k]);
            }
            _ => unreachable!(),
        }
    }
}
