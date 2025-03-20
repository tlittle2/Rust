use std::io::stdin;

fn main() {
    let mut ip1 = String::new();
    let mut ip2 = String::new();

    stdin().read_line(&mut ip1).expect("can't find value from console");
    stdin().read_line(&mut ip2).expect("can't find value from console");

    let x: i32 = ip1.trim().parse().expect("not an int");
    let y: i32 = ip2.trim().parse().expect("not an int");

    println!("{}", y+x);

}
