use proconio::input;

fn main() {
    input! {
        n: u32,
        txy: [(i32,i32,i32);n]
    }

    let mut ok = true;
    let mut t = 0;
    let mut x = 0;
    let mut y = 0;
    for txy_i in txy {
        let time = txy_i.0 - t;
        let dist = (x - txy_i.1).abs() + (y - txy_i.2).abs();
        if time < 0 || dist > time || (time - dist) % 2 != 0 {
            ok = false;
            break;
        }
        t = txy_i.0;
        x = txy_i.1;
        y = txy_i.2;
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
