//! This is the second week of the course.
//! 

use std::sync::{Arc,Mutex};
use std::thread;
use rand::Rng;
use std::collections::HashMap;



fn main(){
    // threading();
    (_,_) = homophonic_cipher("plaintext");
}

// Using threads and mutex
fn threading(){
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let mut handles= vec![];
    for i in 0..3 {
        let data = data.clone();
        handles.push(thread::spawn(move ||{
            let mut data = data.lock().unwrap();
            data[i] += 1;
        }));
    }

    // println!("{:?}", handles);

    for handle in handles{
        handle.join().unwrap();
    }

    println!("{:?}", data);


}

// Playing with HashMap to create homophonic cipher
fn homophonic_cipher(plaintext: &str) -> (String, HashMap<char, Vec<char>>){
    let mut rng = rand::thread_rng();
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut ciphertext = String::new();
    let mut mapping: HashMap<char, Vec<char>> = HashMap::new();

    for c in &alphabet{
        let homophones: Vec<char> = (0..rng.gen_range(2..4))
        .map(|_| rng.gen_range('a'..='z'))
        .collect();
        mapping.insert(*c, homophones);
    }

    println!("{:?}", mapping);

    for c in plaintext.chars(){
        if let Some(c) = c.to_lowercase().next(){
            if let Some(homophones) = mapping.get(&c){
                if let Some(&homophone) = homophones.get(rng.gen_range(0..homophones.len())){
                    ciphertext.push(homophone);
                }
                else{
                    println!("ERROR! No homophone for letter {}", c)
                }
            }
        } else {
            ciphertext.push(c);
        }
    }

    println!("{}\t \t {}", plaintext, ciphertext);

    (ciphertext, mapping)
}

// 








