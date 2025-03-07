use proconio::input;

fn main() {
    input! {
        mut x: i64,
        mut y: i64,
        mut z: i64,
    }

    if x < 0 {
        x = -x;
        y = -y;
        z = -z;
    }

    if y < 0 || x < y || (0 < z && z < y && y < x) {
        println!("{}", x);
    } else if z < 0 {
        println!("{}", x + 2 * -z);
    } else {
        println!("{}", -1);
    }
}
