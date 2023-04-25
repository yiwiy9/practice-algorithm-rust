use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        queries: [(usize,usize,usize); q],
    }

    let mut start_i = 0;

    for (t, mut x, mut y) in queries {
        match t {
            1 => {
                x -= 1;
                x = (x + n - start_i) % n;
                y -= 1;
                y = (y + n - start_i) % n;
                a.swap(x, y);
            }
            2 => {
                start_i += 1;
                start_i %= n;
            }
            3 => {
                x -= 1;
                println!("{}", a[(x + n - start_i) % n]);
            }
            _ => unreachable!(),
        }
    }
}
