use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s_ref: &str = &s;

    match s_ref {
        "tourist" => println!("{}", 3858),
        "ksun48" => println!("{}", 3679),
        "Benq" => println!("{}", 3658),
        "Um_nik" => println!("{}", 3648),
        "apiad" => println!("{}", 3638),
        "Stonefeang" => println!("{}", 3630),
        "ecnerwala" => println!("{}", 3613),
        "mnbvmar" => println!("{}", 3555),
        "newbiedmy" => println!("{}", 3516),
        "semiexp" => println!("{}", 3481),
        _ => unreachable!(),
    }
}
