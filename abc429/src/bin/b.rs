use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let a_sum = a.iter().sum::<usize>();
    if a_sum < m {
        println!("No");
    } else {
        println!(
            "{}",
            if a.iter().find(|&&v| v == a_sum - m).is_some() {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
