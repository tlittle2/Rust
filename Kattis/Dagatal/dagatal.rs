use std::io::stdin;
use std::collections::{self};

fn process_input(mut ip: String) -> i32{
    stdin().read_line(&mut ip).expect("can't find value from stdin");

    let x: i32 = ip.trim().parse().expect("Can't find associated type");

    return x;

}

fn main() {
    let days = collections::HashMap::from([
        (1 , 31),
        (2 , 28),
        (3 , 31),
        (4 , 30),
        (5 , 31),
        (6 , 30),
        (7 , 31),
        (8 , 31),
        (9 , 30),
        (10, 31),
        (11, 30),
        (12, 31)
    ]);

    let x = process_input(String::new());

    println!("{}",days[&x])

}
