use std::io;
fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);

    let _: i32 = buf.trim().parse().expect("not an int");

    let _ = io::stdin().read_line(&mut buf);

    let ip: Vec<i32> = buf
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    buf.clear();

    let f: &Vec<&i32> = &ip.iter().filter(|&x| *x < 0).collect();

    println!("{:?}", f.len());
}
