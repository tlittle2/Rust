use std::io::stdin;

fn process_i32input(mut ip: String) -> i32 {
    stdin().read_line(&mut ip).expect("can't find value from stdin");

    let x: i32 = ip.trim().parse().expect("Can't find associated type");

    return x;

}

fn process_input(mut ip: String) -> String {
    stdin().read_line(&mut ip).expect("can't find value from stdin");

    let x: String = ip.trim().parse().expect("Can't find associated type");

    return x;

}

fn main() {
    let name = process_input(String::new());
    let occurrences = process_i32input(String::new());


    for _ in 0..occurrences{
        println!("Hipp hipp hurra, {}!", name);
    }

}
