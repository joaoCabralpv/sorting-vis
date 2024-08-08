use std::vec::Vec;
pub struct Array {
    array: Vec<i64>,
    n_reads: u128,
    n_writes: u128,
}


impl Array{
    pub fn get_element(&mut self,index:usize) -> i64 {
        self.n_reads+=1;
        self.array[index].clone()
    }
    pub fn set_element(&mut self,index:usize,element:i64) {
        self.n_writes+=1;
        self.array[index] = element;
    }
    pub fn new(array:Vec<i64>) -> Array {
        Array { array: array, n_reads: 0, n_writes: 0 }
    }
    pub fn get_array(&self) -> &Vec<i64>{
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
}