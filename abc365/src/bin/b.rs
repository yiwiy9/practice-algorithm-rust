use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut first = 0;
    let mut first_i = 0;
    let mut second = 0;
    let mut second_i = 0;
    for (i, &a_i) in a.iter().enumerate() {
        if a_i >= first {
            second = first;
            first = a_i;
            second_i = first_i;
            first_i = i;
        } else if a_i >= second {
            second = a_i;
            second_i = i;
        }
    }

    println!("{}", second_i + 1);
}
