use std::collections::HashSet;

fn main() {
    let bad_set: HashSet<char> = "e".chars().collect();
    let sentence_list: Vec<String> = "we need to improve synergy through team building exercises".split_whitespace().map(String::from).collect();

    println!("{:?}", sentence_list);
    println!("{:?}", bad_set);

    let answer: Vec<String> = sentence_list.iter().fold(Vec::new(), |mut acc, word|{
        if word.chars().any(|c| bad_set.contains(&c)){
            acc.push("*".repeat(word.len()));
        }else{
            acc.push(word.clone());
        }
        acc
    });
    println!("{:?}", answer);
}
