use std::error::Error;

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
               if !container.is_empty() && !isKeyWord(container.as_str()) {
                    opnds += 1;
               }
               container.clear();
          }
          i += 1;
     }
     if !container.is_empty() && !isKeyWord(container.as_str()) {
          opnds += 1;
     }
     Some((ops, opnds))
}






#[cfg(test)]
mod test {
     use super::*;
     #[test]
     fn test() {
          assert_eq!(true, isKeyWord("int"))
     }
     #[test]
    fn test2() {
         let s = "s2n(TLSEXT_TYPE_status_request,ret);";
        println!("{:?}", recognize(s));  
          
     }
}
