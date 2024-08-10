mod array;
use macroquad::prelude::*;
use array::Array;
use std::vec::Vec;
use rand::RandomRange;


async fn sort(mut array:Array)-> Array{
    let mut sorted = false;
    let mut n_rounds:usize = 0;
    while !sorted {
        sorted = true;
        n_rounds+=1;
        for i in 0..(array.len()-n_rounds){
            if array.get_element(i).await > array.get_element(i+1).await{
                let e0 = array.get_element(i).await;
                let e1 = array.get_element(i+1).await;
                array.set_element(i, e1).await;
                array.set_element(i+1,e0).await;
                sorted  = false;
            }
        }
    }
    array
}


#[macroquad::main("sorting vis")]

async fn main() {
    request_new_screen_size(1280.0, 720.0);
    
    rand::srand(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Please use this program after 00:00:00 UTC 0 January 1st 1970").as_secs());

    let mut vec = Vec::new();
    for _ in 0..1000 {
        vec.push(RandomRange::gen_range(0, 3000))
    }
    println!("1");
    let mut arr = Array::new(vec);
    let t = std::time::SystemTime::now();
    arr = sort(arr).await;
    println!("Time: {}s",std::time::SystemTime::now().duration_since(t).expect("Time went backwards").as_secs_f64());
    println!("Reads: {}",arr.get_reads());
    println!("Writes: {}", arr.get_writes());
}