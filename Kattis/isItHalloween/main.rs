use std::io;

fn main() {
    let holidays = vec!["OCT 31", "DEC 25"];

    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf).unwrap();
    let x = buf.trim();

    if holidays.contains(&x){
        println!("yup");
    }else{
        println!("nope");
    }
}
