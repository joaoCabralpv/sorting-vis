use core::f32;
use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

use macroquad::color;
use macroquad::prelude::*;

enum AccessType {
    Reading,
    Writing

}
pub struct Array {
    array: Vec<i64>,
    accessing:usize,
    access_type: AccessType,
    n_reads: u128,
    n_writes: u128,
}


impl Array{
    pub async fn get_element(&mut self,index:usize) -> i64 {
        self.n_reads+=1;
        self.accessing = index;
        self.access_type = AccessType::Reading;
        self.render().await;
        self.array[index].clone()
    }
    pub async fn set_element(&mut self,index:usize,element:i64) {
        self.n_writes+=1;
        self.accessing = index;
        self.access_type = AccessType::Writing;
        self.array[index] = element;
        self.render().await;
    }
    pub fn new(array:Vec<i64>) -> Array {
        Array { array: array, accessing: 0, access_type: AccessType::Reading, n_reads: 0, n_writes: 0 }
    }
    pub async fn get_array(&self) -> &Vec<i64>{
        &self.array
    }
    pub fn len(&self) -> usize{
        self.array.len()
    }
    pub fn get_reads(&self) -> u128 {
        self.n_reads
    }
    pub fn get_writes(&self) -> u128 {
        self.n_writes
    }
    pub async fn render(&mut self) {
        let rect_width = screen_width() / self.array.len() as f32;
        let rect_scale = screen_height()  / self.array.iter().max().expect("Array is empty").clone() as f32;
        for i in 0..self.array.len() {
            let color = match i == self.accessing {
                true => match self.access_type {
                    AccessType::Reading => GREEN,
                    AccessType::Writing => RED,
                },
                false => color::WHITE,
            };

            draw_rectangle((i as f32*rect_width) as f32, screen_height()-(rect_scale*self.array[i]as f32), rect_width as f32 , f32::MAX, color); //Why calculate the height when you can just use f32::MAX lol
        }
        next_frame().await;
        //println!("{:?}",self.array);
    }
}