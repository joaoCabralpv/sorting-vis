use std::vec::Vec;
pub struct Array<T> {
    array: Vec<T>,
    n_reads: u128,
    n_writes: u128,
}


impl<T> Array<T> {
    pub fn get_element(&mut self,index:usize) -> &T {
        self.n_reads+=1;
        &self.array[index]
    }
    pub fn set_element(&mut self,index:usize,element:T) {
        self.n_writes+=1;
        self.array[index] = element;
    }
    pub fn new(array:Vec<T>) -> Array<T> {
        Array { array: array, n_reads: 0, n_writes: 0 }
    }
    pub fn get_array(&self) -> &Vec<T>{
        &self.array
    }
}