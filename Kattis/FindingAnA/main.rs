use std::io;

fn main() {

    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf).unwrap();
    let x = buf.trim();

    let idx = x.find('a').unwrap();

    println!("{}", &x[idx..x.len()])
}
