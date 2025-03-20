use std::io::stdin;

fn process_input(mut ip: String) -> i32{
    stdin().read_line(&mut ip).expect("can't find string from stdin");

    let x: i32 = ip.trim().parse().expect("Can't parse the string");

    return x;

}

fn main() {
    let x = process_input(String::new());

    if x % 10 == 0 {
        println!("Jebb");
    }else{
        println!("Neibb");
    }

}
