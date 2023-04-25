use proconio::input;

fn main() {
    input! {
        h: f64,
        w: f64,
    }

    if h == 1.0 {
        println!("{}", w);
        return;
    }

    if w == 1.0 {
        println!("{}", h);
        return;
    }

    println!("{}", (h / 2.0).ceil() * (w / 2.0).ceil());
}
