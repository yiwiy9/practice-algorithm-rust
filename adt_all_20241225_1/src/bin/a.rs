use proconio::input;

fn main() {
    input! {
        s: [usize; 8],
    }

    let mut ok = true;
    for (i, &s_i) in s.iter().enumerate() {
        if i > 0 && s[i - 1] > s_i {
            ok = false;
        }

        if s_i < 100 || s_i > 675 {
            ok = false;
        }

        if s_i % 25 != 0 {
            ok = false;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
