use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut map = std::collections::HashMap::new();
    for _ in 0..q {
        input! {
            op: u8,
        }

        match op {
            1 => {
                input! {
                    x: i64,
                }
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: i64,
                }
                let cur = *map.get(&x).unwrap_or(&0);
                if cur <= 1 {
                    map.remove(&x);
                } else {
                    *map.get_mut(&x).unwrap() -= 1;
                }
            }
            3 => {
                println!("{}", map.len());
            }
            _ => unreachable!(),
        }
    }
}
