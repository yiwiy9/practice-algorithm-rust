use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let t_set = t.into_iter().collect::<std::collections::HashSet<_>>();

    println!(
        "{}",
        s.iter()
            .map(|x| {
                if t_set.contains(x) {
                    "Yes"
                } else {
                    "No"
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
}
