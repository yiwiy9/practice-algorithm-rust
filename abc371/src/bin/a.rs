use proconio::input;

fn main() {
    input! {
        s_ab: char,
        s_ac: char,
        s_bc: char,
    }

    if s_ab == '<' {
        if s_ac == '<' {
            if s_bc == '<' {
                println!("B");
            } else {
                println!("C");
            }
        } else {
            println!("A");
        }
    } else {
        if s_ac == '<' {
            println!("A");
        } else {
            if s_bc == '<' {
                println!("C");
            } else {
                println!("B");
            }
        }
    }
}
