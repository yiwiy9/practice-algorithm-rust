use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut m: usize,
    }

    loop {
        let a = h / 10;
        let b = h % 10;
        let c = m / 10;
        let d = m % 10;

        let new_h = a * 10 + c;
        let new_m = b * 10 + d;
        if new_h < 24 && new_m < 60 {
            println!("{} {}", h, m);
            return;
        }

        m += 1;
        if m == 60 {
            m = 0;
            h += 1;
            if h == 24 {
                h = 0;
            }
        }
    }
}
