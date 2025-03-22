use std::io::stdin;

fn process_i32input(mut ip: String) -> i32{
    stdin().read_line(&mut ip).expect("can't find string from stdin");

    let x: i32 = ip.trim().parse().expect("Can't find an i32");

    return x;

}

fn main() {
    let x = process_i32input(String::new());
    let y = process_i32input(String::new());

    println!("{}", match (x,y){
        (a,b) if a > b => "MAGA!", 
        (a,b) if a < b => "FAKE NEWS!",
        _ => "WORLD WAR 3!"
    })

}
