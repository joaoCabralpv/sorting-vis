mod array;
use macroquad::prelude::*;
use array::Array;
use std::{time, vec::Vec};
use rand::RandomRange;


fn sort(mut array:Array)-> Array{
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..array.len()-1{
            if array.get_element(i) > array.get_element(i+1){
                let e0 = array.get_element(i);
                let e1 = array.get_element(i+1);
                array.set_element(i, e1);
                array.set_element(i+1,e0);
                sorted  = false;
            }
        }
    }
    array
}


#[macroquad::main("sorting vis")]

async fn main() {
    //request_new_screen_size(1280.0, 720.0);
    
    rand::srand(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Please use this program after 00:00:00 UTC 0 January 1st 1970").as_secs());

    let mut vec = Vec::new();
    for _ in 0..100000 {
        vec.push(RandomRange::gen_range(0, 100000))
    }
    println!("1");
    let mut arr = Array::new(vec);
    let t = std::time::SystemTime::now();
    arr = sort(arr);
    println!("Time: {}s",std::time::SystemTime::now().duration_since(t).expect("Time went backwards").as_secs_f64());
    println!("Reads: {}",arr.get_reads());
    println!("Writes: {}", arr.get_writes());
}