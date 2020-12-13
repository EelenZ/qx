use std::{sync::Mutex, collections::HashMap};
use crate::metric::{*};
use crate::util::{isKeyWord};


pub struct Counter{
    metadata: HashMap<String, usize>,

}



impl Counter {
    pub fn new() -> Self {
        let metadata = HashMap::new();
        Self {
            metadata,
        }
    }
    pub async fn count(&mut self, lines: &Vec<Vec<&str>>) {
        let LOC: usize = lines.len() - 2;
        let mut ops: usize = 0;
        let mut opnds: usize = 0;
        for items in &lines[1..(LOC+1)] {
            for item in items {
                let s = *item;
                todo!()
         

            }   
        }
    }
    
}


#[cfg(test)]
mod test {
    use super::*;

    #[async_std::test]
    async fn test1() {
       
    }
}