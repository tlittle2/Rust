use std::io;
fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    let ip: Vec<f32> = buf
        .split_whitespace()
        .map(|x| x.parse::<f32>().unwrap())
        .collect();
    buf.clear();

    let maxweight = (ip[0] - ip[1]) * 0.9;

    let _ = io::stdin().read_line(&mut buf);
    let ip2: Vec<f32> = buf
        .split_whitespace()
        .map(|x| x.parse::<f32>().unwrap())
        .collect();
    buf.clear();

    let s: f32 = ip2.iter().sum();


    println!("{}", maxweight - s);

}
