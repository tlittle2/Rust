use std::io::stdin;

fn process_input(mut ip: String) -> i32{
    stdin().read_line(&mut ip).expect("can't find string from stdin");

    let x: i32 = ip.trim().parse().expect("not an int");

    return x;

}

fn main() {

    let x = process_input(String::new());
    let y =  process_input(String::new());
    let z  =  process_input(String::new());

    println!("{}", z-(y+x));

}
