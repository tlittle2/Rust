use std::io;

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);

    let values: Vec<i32> = buf
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    buf.clear();

    let _ = io::stdin().read_line(&mut buf);
    let bags: Vec<i32> = buf
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let comparor = *values.get(1).unwrap();

    if comparor == *bags.get(0).unwrap() {
        println!("{}", "fyrst");
    } else if comparor == *bags.get(1).unwrap() {
        println!("{}", "naestfyrst");
    } else {
        println!("{} fyrst", bags.iter().position(|x| *x == comparor).unwrap() + 1);
    }
}
