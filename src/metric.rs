use std::collections::HashMap;

pub struct Metrics {
    //traditional metrics
    LOC: usize, // line of code
    total_Op: usize, //  total operators
    total_Opnd: usize, // total operands
    
    //CDG metrics


}

impl Metrics {
    pub fn new(metadata: HashMap<&str, usize>) -> Self{
        todo!()
    }
}