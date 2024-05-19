use proconio::input;

fn main() {
    input! {
        h: usize,
    }

    let mut p_h = 0;
    for i in 0..=60 {
        p_h += 2_usize.pow(i);
        if p_h > h {
            println!("{}", i + 1);
            break;
        }
    }
}
