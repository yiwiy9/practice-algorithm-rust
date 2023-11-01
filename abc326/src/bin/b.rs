use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in n..1000 {
        let h = i / 100;
        let t = (i / 10) % 10;
        let o = i % 10;
        if h * t == o {
            println!("{}", i);
            return;
        }
    }
}
