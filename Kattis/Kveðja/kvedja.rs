use std::io::stdin;

fn process_input(mut ip: String) -> String{
    stdin().read_line(&mut ip).expect("can't find string from stdin");

    let x: String = ip.trim().parse().expect("Can't parse the string");

    return x;

}

fn main() {
    let name = process_input(String::new());
    println!("{}{}","Kvedja,\n", name);

}
