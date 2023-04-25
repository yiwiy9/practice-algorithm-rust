use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [i32; n],
    }

    a.sort_by(|a, b| b.cmp(a));

    let mut alice = 0;
    let mut bob = 0;

    for (i, val) in a.iter().enumerate() {
        match i % 2 {
            0 => alice += val,
            _ => bob += val,
        }
    }

    println!("{}", alice - bob);
}
