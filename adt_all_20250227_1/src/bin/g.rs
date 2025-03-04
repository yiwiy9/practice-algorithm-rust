use proconio::input;

fn main() {
    input! {
        t: usize,
        a_s: [(usize,usize); t],
    }

    // x + y = (両方が立っているビット * 2) + どちらかが立っているビット
    //       = 2(x&y) + (x^y)
    // よって、x^y = s - 2*a
    // => (s - 2*a) & a = x^y & x&y = 0

    for (a, s) in a_s {
        println!(
            "{}",
            if s >= 2 * a && (s - 2 * a) & a == 0 {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
