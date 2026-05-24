use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let a = buf.trim().to_string();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let b = buf.trim().to_string();

    print!("{}", a);
    print!("{}", b);
}
