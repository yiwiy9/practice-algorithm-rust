use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize
    }

    let min_r = r.min(15 - r + 1);
    let min_c = c.min(15 - c + 1);
    if min_r % 2 == 1 {
        if min_c % 2 == 0 && min_c < min_r {
            println!("white");
        } else {
            println!("black");
        }
    } else {
        if min_c % 2 == 1 && min_c < min_r {
            println!("black");
        } else {
            println!("white");
        }
    }
}
