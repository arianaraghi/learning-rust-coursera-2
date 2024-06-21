//! This is the second week of the course.
//! 

use std::sync::{Arc,Mutex};
use std::thread;



fn main(){
    // threading();
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

// 



//








