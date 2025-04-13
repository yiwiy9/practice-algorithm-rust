use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut is_logged_in = false;
    let mut ans = 0;
    for i in 0..n {
        if s[i] == "login" {
            is_logged_in = true;
        } else if s[i] == "logout" {
            is_logged_in = false;
        } else if s[i] == "private" && !is_logged_in {
            ans += 1;
        }
    }

    println!("{}", ans);
}
