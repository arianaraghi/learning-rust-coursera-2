//! This is the second week of the course.
//! 

use std::{error::Error, io, process};
use std::sync::{Arc,Mutex};
use std::thread;
use rand::Rng;
use std::collections::HashMap;
use rayon::prelude::*;
use wikipedia::http::default::Client;
use wikipedia::Page;
use wikipedia::Wikipedia;




fn main(){
    // threading();
    // (_,_) = homophonic_cipher("plaintext");
    // webcrawling();
    // process_csv();
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

// Webcrawling using Rayon
struct ProcessedPage {
    title: String,
    data: String,
}
const PAGES: [&str; 9] = [
    "Giannis Antetokounmpo",
    "James Harden",
    "Russell Westbrook",
    "Stephen Curry",
    "Kevin Durant",
    "LeBron James",
    "Kobe Bryant",
    "Michael Jordan",
    "Shaquille O'Neal",
];
fn process_page(page: &Page<Client>) -> ProcessedPage {
    let title = page.get_title().unwrap();
    let content = page.get_content().unwrap();
    ProcessedPage {
        title,
        data: content,
    }
}
fn webcrawling(){
    let start = std::time::Instant::now();
    let wikipedia = Wikipedia::<Client>::default();
    let pages: Vec<_> = PAGES
        .par_iter() //parallel iterator
        .map(|&p| wikipedia.page_from_title(p.to_string()))
        .collect();

    let processed_pages: Vec<ProcessedPage> = pages.par_iter().map(process_page).collect();
    for page in processed_pages {
        //time how long it takes to process each page
        let start_page = std::time::Instant::now();

        println!("Title: {}", page.title.as_str());
        //grab first sentence of the page
        let first_sentence = page.data.split('.').next().unwrap();
        println!("First sentence: {}", first_sentence);
        //count the number of words in the page
        let word_count = page.data.split_whitespace().count();
        println!("Word count: {}", word_count);
        //prints time it took to process each page
        println!("Page time: {:?}", start_page.elapsed());
    }
    //descriptive statistics of: total time, average time per page, and total number of pages, as well as the number of threads used
    println!("Total time: {:?}", start.elapsed());
    println!(
        "Average time per page: {:?}",
        start.elapsed() / PAGES.len() as u32
    );
    println!("Total number of pages: {}", PAGES.len());
    println!("Number of threads: {}", rayon::current_num_threads());
}

// Processing CSV files
fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records(){
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
fn process_csv(){
    if let Err(err) = example(){
        println!("Error running example: {}", err);
        process::exit(1)
    }
 }

 /* 
    Here I should have practiced Cargo Lambda to play with AWS file systems.
    However, since I live in a restricted region and country, I cannot use any of the 
    AWS systems; therefore, I could not practice any Cargo Lambda functionality.
*/








