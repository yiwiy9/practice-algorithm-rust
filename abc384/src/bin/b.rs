use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: i64,
        da: [(u8,i64); n],
    }

    for (t, a) in da {
        match t {
            1 => {
                if 1600 <= r && r <= 2799 {
                    r += a;
                }
            }
            2 => {
                if 1200 <= r && r <= 2399 {
                    r += a;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", r);
}
