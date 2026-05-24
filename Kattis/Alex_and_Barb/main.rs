fn main() {
    let ip: Vec<i32> = "25 3 10".split_whitespace().map(|s| s.parse().expect("not able to parse")).collect();

    unsafe {
        if ip.get_unchecked(0) % (ip.get_unchecked(1) + ip.get_unchecked(2) ) < *ip.get_unchecked(1){
            println!("Barb");
        }else{
            println!("Alex");
        }
    }
}
