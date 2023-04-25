use proconio::input;

fn main() {
    input! {
        n: u8,
        mut a:[i32;n],
    }

    let mut count = 0;
    let mut ok = true;
    loop {
        for el in &mut a {
            if *el % 2 != 0 {
                ok = false;
                break;
            }
            *el /= 2;
        }

        if ok {
            count += 1;
        } else {
            break;
        }
    }

    println!("{}", count);
}
