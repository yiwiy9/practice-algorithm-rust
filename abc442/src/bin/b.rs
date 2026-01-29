use proconio::input;

fn main() {
    input! {
        q: usize,
        a: [usize; q],
    }

    let mut loudness: usize = 0;
    let mut is_played = false;
    for &a_i in &a {
        if a_i == 1 {
            loudness += 1;
        } else if a_i == 2 {
            loudness = loudness.saturating_sub(1);
        } else {
            is_played = !is_played;
        }

        println!(
            "{}",
            if loudness >= 3 && is_played {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
