use nom::{IResult, bytes::complete::tag, character::{complete::alphanumeric1, is_alphabetic}, sequence::{pair, tuple}};

///KeyWord table
const kwtb: [&str; 32] = ["int", "char", "double", "float", "short", "long", "auto", "break",
                        "case", "const", "continue", "default", "do", "else", "enum", "extern",
                        "for", "goto", "if", "register", "return", "signed", "sizeof", "static",
                        "struct", "switch", "typedef", "unsigned", "union", "void", "volatile", "while"
];


pub async fn isKeyWord(s: &str) -> bool {
     for item in kwtb.into_iter() {
          if *item == s {
               return true
          }
     }
     false
}

fn match_func(s: &str) -> IResult<&str, &str> {
    // pair(alphanumeric1, tag("("))
    todo!()
}


pub async fn recongnize(s: &str) -> Option<(usize, usize)>{
    
    if let Ok((rest, matched)) = match_func(s) {

    }
    
     todo!()
}


#[cfg(test)]
mod test {
     use super::*;
     #[async_std::test]
     async fn test() {
          assert_eq!(true, isKeyWord("int").await)
     }
     #[async_std::test]
     async fn test2() {
          assert_eq!(false, isKeyWord("tin").await);
     }
}
