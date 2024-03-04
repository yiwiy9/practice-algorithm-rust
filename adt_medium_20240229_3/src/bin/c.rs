use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String,String); n],
    }

    let mut ok = true;
    for i in 0..n {
        let mut first_ok = true;
        let mut second_ok = true;
        for j in 0..n {
            if i == j {
                continue;
            }
            if st[i].0 == st[j].0 || st[i].0 == st[j].1 {
                first_ok = false;
            }
            if st[i].1 == st[j].0 || st[i].1 == st[j].1 {
                second_ok = false;
            }
        }
        if !first_ok && !second_ok {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
