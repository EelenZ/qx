use std::{error::Error};
use async_std::{self, fs::File, fs::read_dir, io::{BufReader, prelude::BufReadExt}, path::Path, path::PathBuf};
use async_std::stream::StreamExt;
use std::{sync::Mutex, collections::HashMap};

use crate::count::Counter;
use crate::metric::*;

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;


pub struct CDG {
    filename: String,
    label: bool,
    metrics: Metrics,
}

impl CDG {

    pub fn new(metadata: HashMap<&str, usize>, filename: String, label: bool)-> Self{
        let mut metircs = Metrics::new(metadata);
        Self {
            filename,
            label,
            metrics:metircs,
        }
    }

    pub fn save(&self) -> Result<()> {

        
        Ok(())
    }
    
}

pub struct Parser {
}

impl Parser {

    async fn read_CDG(file_path: &Path) -> Result<()> {
        let file = BufReader::new(File::open(file_path).await?);
        let mut lines = file.lines();
        
        let mut codelines = vec![];
        while let Some(line) = lines.next().await {
            let line = line?;
            if  !line.starts_with("---"){
                codelines.push(line);
            } else {
                if let Ok(splitedlines) = Self::split(&mut codelines).await{
                    //let mut counter = Counter::new();
                    //let metadata = counter.count(&splitedlines);
                    let filename = String::from(splitedlines[0][1]);
                    //let lalel = splitedlines[][0];
                    let mut metadata:HashMap<&str, usize> = HashMap::new();
                    metadata.insert("LOC", 10);
                    metadata.insert("total_Op", 10);
                    metadata.insert("total_Opnd", 10);
                    let label = true;
                    let cdg = CDG::new(metadata, filename, label);
                    cdg.save();

                }
                
                codelines.clear();
            }
        }

        Ok(())
    }

    pub async fn split(cdg: & mut Vec<String>) -> Result<Vec<Vec<&str>>>{
        let splitedlines: Vec<Vec<&str>> = cdg.into_iter().map(|line|-> Vec<&str>{
            line.split(' ').collect()
        }).collect();
        //println!("{:#?}", cdg);
        Ok(splitedlines)
    }

    pub async fn format(cdg: & mut Vec<String>) -> Result<Vec<String>> {
        let formatlines: Vec<String> = cdg.into_iter().map(|line|-> Vec<&str>{
            line.split(' ').collect()
        }).map(|items|-> String {
            items.join("")
        }).collect();
        Ok(formatlines)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[async_std::test]
    async fn test() {
        let mut path = PathBuf::from("data\\CDG.txt");
     
       
    }

    #[async_std::test]
    async fn test2() {
        let mut cdg:Vec<String> = vec![
            "1 CVE-2016-6304/OpenSSL_1.0.1_CVE-2016-6304_t1_lib.c cfunc 813".to_string(),
            "unsigned char *ssl_add_serverhello_tlsext(SSL *s, unsigned char *p, unsigned char *limit)".to_string(),
            "unsigned char *ret = p;".to_string(),
            "ret+=2;".to_string(),
            "s2n(TLSEXT_TYPE_server_name,ret);".to_string(),
            "s2n(0,ret);".to_string(),
            "int el;".to_string(),
            "if(!ssl_add_serverhello_renegotiate_ext(s, 0, &el, 0))".to_string(),
            "s2n(TLSEXT_TYPE_renegotiate,ret);".to_string(),
            "0".to_string(),
        ];
        let cdg = Parser::format(&mut cdg).await.unwrap();
        println!("{:?}", cdg);
    }
}