use std::io::stdin;

fn process_i32input(mut ip: String) -> i32{
    stdin().read_line(&mut ip).expect("can't find string from stdin");

    let x: i32 = ip.trim().parse().expect("Can't find an i32");

    return x;

}

fn process_string_input(mut ip: String) -> String{
    stdin().read_line(&mut ip).expect("can't find string from stdin");

    let x: String = ip.trim().parse().expect("Can't parse the string");

    return x;

}

fn main() {
    let word = process_string_input(String::new());
    let occur = process_i32input(String::new());

    for _ in 1..occur+1{
        print!("{}", word);
    }


}
