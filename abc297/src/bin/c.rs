use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        room: [Chars; h],
    }

    for row in &room {
        let mut before_c = '*';
        for c in row {
            if *c == 'T' && before_c == 'T' {
                print!("P");
                before_c = 'C';
            } else {
                if before_c != '*' {
                    print!("{}", before_c);
                }
                before_c = *c;
            }
        }
        println!("{}", before_c);
    }
}
