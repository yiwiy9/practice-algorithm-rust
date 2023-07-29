use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let correct_s = vec!["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];

    for &s_i in &correct_s {
        if s_i == s {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
