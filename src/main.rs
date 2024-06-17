//! This is a testing program to get to know more about all the features in Rust through a Coursera course.
//!

use rand::seq::SliceRandom;
use rand::thread_rng;
// use std::collections::linked_list;
use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::fmt::Debug;

fn main() {
    // vectoring();
    // vec_dequing();
    // linkedListing();
    // hashmapping();
}

fn vectoring() {
    let mut fruit = vec!["Apple", "Orange", "Peach", "Pear", "Fig", "Cherry"];

    // Shuffle the friut vector
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // Printing fruit salad
    println!("Fruit salad is:");
    for el in fruit.iter() {
        println!("{}", el);
    }
}
fn vec_dequing() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbotus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Shuffling fruits
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Back to VeqDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Adding more items
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Printing fruit salad
    println!("Fruit salad is:");
    for el in fruit.iter() {
        println!("{}", el);
    }
}
fn linkedListing() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbotus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Shuffling fruits
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Adding more items
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Printing fruit salad
    println!("Fruit salad is:");
    for el in fruit.iter() {
        println!("{}", el);
    }
}
fn hashmapping(){
    let numbers = vec![1, 2, 3, 4, 5, 3,2, 3, 5,6, 43,3, 2,3, 5,3, 2,2, 3, 5,3 ];
    let freqs = logic(numbers);
    println!("{:?}", freqs)
}
fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies: HashMap<i32, u32> = HashMap::new();

    for num in numbers{
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency +=1;
    }

    let mut result: Vec<(i32, u32)> = Vec::new();

    for (num, freq) in frequencies{
        result.push((num, freq));
    }

    result.sort();

    result

}





