use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                let mut combined = s[i].clone();
                combined.push_str(&s[j]);
                set.insert(combined);
            }
        }
    }

    println!("{}", set.len());
}
