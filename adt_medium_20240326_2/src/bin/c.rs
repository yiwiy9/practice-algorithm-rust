use proconio::input;

fn main() {
    input! {
        mut a: i64,
        mut m: i64,
        mut l: i64,
        mut r: i64,
    }

    let l_a = (a - l).abs() / m;
    let r_a = (a - r).abs() / m;

    if a < l {
        println!("{}", r_a - l_a + if (a - l).abs() % m == 0 { 1 } else { 0 });
    } else if a <= r {
        println!("{}", l_a + r_a + 1);
    } else {
        println!("{}", l_a - r_a + if (a - r).abs() % m == 0 { 1 } else { 0 });
    }
}
