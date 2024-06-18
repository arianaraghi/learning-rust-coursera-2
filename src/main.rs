//! This is a testing program to get to know more about all the features in Rust through a Coursera course.
//!

use petgraph::visit::IntoEdgesDirected;
use petgraph::Direction::Outgoing;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::fmt::Debug;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;

fn main() {
    // vectoring();
    // vec_dequing();
    // linkedListing();
    // hashmapping();
    // graph_centrality();
    // set_usage();
    btreeset_usage();
}

// Vector, VecDeque, LinkedList, and HashMap
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
    let freqs = hashmap_logic(numbers);
    println!("{:?}", freqs)
}
fn hashmap_logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
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

// Graphs
#[derive(Debug)]
struct Fighter{
    name: String,
}
impl Fighter{
    fn new(name: &str) -> Self{
        Self{
            name: name.to_string()
        }
    }
}
impl fmt::Display for Fighter{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.name)
    }
}
fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a:usize, b:usize){
    graph.add_edge(nodes[a], nodes[b], 1.0);
}
fn graph_centrality(){
    let mut graph: UnGraph<&Fighter, f32> = UnGraph::new_undirected();
    let fighters = [
        Fighter::new("John"),
        Fighter::new("Alex"),
        Fighter::new("Mohammad"),
        Fighter::new("Look"),
        Fighter::new("Manni"),
        Fighter::new("Lukas"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
    .iter()
    .map(|fighter|graph.add_node(fighter))
    .collect();

    add_edge(&mut graph, &fighter_nodes, 1, 0);
    add_edge(&mut graph, &fighter_nodes, 1, 3);
    add_edge(&mut graph, &fighter_nodes, 1, 5);
    add_edge(&mut graph, &fighter_nodes, 4, 2);
    add_edge(&mut graph, &fighter_nodes, 4, 3);
    add_edge(&mut graph, &fighter_nodes, 5, 3);
    add_edge(&mut graph, &fighter_nodes, 5, 2);
    add_edge(&mut graph, &fighter_nodes, 5, 4);
    add_edge(&mut graph, &fighter_nodes, 5, 0);


    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0/degree;
        println!("The CLoseness of {} is {:.2}", name, closeness);
        println!("----------------------------------------");
    }

    

}

// HashSet (Set)
fn generate_fruits() -> &'static str{
    let fruits = vec![
        "Apple", "Orange", "Peach", "Pear", 
        "Fig", "Cherry", "Date", 
        "Elderberry", "Berry", "Grape", 
        "Honeydew", "Melon", "Watermelon"
        ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}
fn set_usage(){
    let mut fruit_set = HashSet::new();
    let mut fruit_vec = Vec::new();
    for _ in 0..=100{
        let temp = generate_fruits();
        fruit_set.insert(temp);
        fruit_vec.push(temp);
    }
    println!("{:?}", fruit_set);
    println!("-------------------------------------------------------");
    println!("{:?}", fruit_vec);
}

// BTreeSet
fn btreeset_usage(){
    let fruits = vec![
        "Apple", "Orange", "Peach", "Pear", 
        "Fig", "Cherry", "Date", 
        "Elderberry", "Berry", "Grape", 
        "Honeydew", "Melon", "Watermelon"
        ];
    let mut rng = thread_rng();
    let amounts = vec![1,3,5,7,9];

    for amount in amounts{
        let mut fruit_set = BTreeSet::new();
        let mut shuufled_fruits = fruits.clone();
        shuufled_fruits.shuffle(&mut rng);

        for fruit in shuufled_fruits{
            fruit_set.insert(fruit);
            if fruit_set.len() >= amount{break}
        }

        println!("{}:\t{:?}", amount, fruit_set);
    }

}

//






