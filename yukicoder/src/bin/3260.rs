use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        n: usize,
        uv: [(i64, i64); n],
    }

    for &(u, v) in &uv {
        if u == 0 {
            println!("{}", if v % y == 0 { y } else { v % y });
        } else if (u - 1) / y == (v - 1) / y {
            println!("{}", v - u);
        } else {
            let u_0 = if u % y == 0 { y } else { u % y };
            let v_0 = if v % y == 0 { y } else { v % y };
            println!("{}", u_0 + v_0);
        }
    }
}
