use std::io::stdin;


fn process_input(mut ip: String) -> String {
    stdin().read_line(&mut ip).expect("can't find value from stdin");

    let x: String = ip.trim().parse().expect("Can't find associated type");

    return x;

}

fn main() {

    let ip = process_input(String::new());

    println!("{}", ip.chars().rev().collect::<String>());
    
}
