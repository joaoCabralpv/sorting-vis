mod array;
use macroquad::prelude::*;
use array::Array;
use std::vec::Vec;
use rand::RandomRange;

#[macroquad::main("sorting vis")]
async fn main() {
    //request_new_screen_size(1280.0, 720.0);
    //rand::srand()
    
    rand::srand(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Please use this program after 00:00:00 UTC 0 January 1st 1970").as_secs());

    let mut vec = Vec::new();
    for _ in 0..100 {
        vec.push(RandomRange::gen_range(0, 100))
    }
    let mut arr = Array::new(vec);
    for i in 0..arr.get_array().len(){
        print!("{},",arr.get_element(i))
    }
    /*loop {
        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }*/

}