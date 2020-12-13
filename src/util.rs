use std::error::Error;

use combine::parser;
//KeyWord table
const kwtb: [&str; 32] = ["int", "char", "double", "float", "short", "long", "auto", "break",
                        "case", "const", "continue", "default", "do", "else", "enum", "extern",
                        "for", "goto", "if", "register", "return", "signed", "sizeof", "static",
                        "struct", "switch", "typedef", "unsigned", "union", "void", "volatile", "while"
];



pub fn isKeyWord(s: &str) -> bool {
     for item in kwtb.into_iter() {
          if *item == s {
               return true
          }
     }
     false
}

pub fn isIdentifier(c: char) -> bool{
     if c >= 'a' && c <='z' 
          || c >= 'A' && c <= 'Z' 
          || c >= '0' && c <= '9'
          || c == '_' {
               return true;
     }
     false
} 

pub fn recognize(s: &str) -> Option<(usize, usize)>{
     let mut ops: usize = 0;
     let mut opnds: usize = 0;
     let mut i: usize = 0;
     let mut container = String::new();
     let cs: Vec<char> = s.chars().collect();
     while i < cs.len() {
          if isIdentifier(cs[i]) {
               container.push(cs[i]);
          }
          else {
               ops += 1;
               if !isKeyWord(container.as_str()) {
                    opnds += 1;
               }
               container.clear();
          }
     }
     todo!()
}






#[cfg(test)]
mod test {
     use super::*;
     #[test]
     fn test() {
          assert_eq!(true, isKeyWord("int"))
     }
     #[async_std::test]
     async fn test2() {
          //assert_eq!(false, isKeyWord("tin").await);
          
     }
}
