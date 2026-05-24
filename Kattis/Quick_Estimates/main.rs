use std::io;

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    let a: i32 = buf.trim().parse().unwrap();
    buf.clear();

    for _ in 0..a{
        let _ = io::stdin().read_line(&mut buf);
        let i: String = buf.trim().parse().unwrap();
        println!("{:?}", i.len());
        buf.clear();
    }
}
