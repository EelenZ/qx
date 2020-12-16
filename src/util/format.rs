use std::error::Error;

use async_std::{path::PathBuf,
                path::Path,
                fs::OpenOptions,
                fs::File,
                fs::read_dir,
                io::{BufReader, BufWriter, prelude::BufReadExt},
};
use futures::StreamExt;

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;

pub async fn read_project(root: PathBuf) -> Result<()> {
    let mut path_list: Vec<PathBuf> = Vec::new();
    let mut file_list: Vec<PathBuf> = Vec::new();
    path_list.push(root);

    while let Some(path) = path_list.pop() {
        if path.is_file().await {
            file_list.push(path);
        }
        else if path.is_dir().await {
            let mut dir = read_dir(path).await?;
            while let Some(res) = dir.next().await {
                let path = res?.path();
                if path.is_dir().await {
                    path_list.push(path);
                }
                else if path.is_file().await{
                    file_list.push(path);
                }
            }
        }    
    }  
    Ok(())
}

pub async fn format( file_list: &Vec<PathBuf>)-> Result<()>{
    for file_path in file_list {
        let mut file = BufReader::new(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .await?
        );
        format_file(&mut file).await;
    }
    Ok(())
}

pub async fn format_file(file: &mut BufReader<File>) -> Result<()>{
    while let Some(line) = file.lines().next().await {
        let line = line?;
        format_line(line).await;
    }
    Ok(())
}

pub async fn format_line(line: String) -> Result<()> {
    let line = line.trim();
    
    Ok(())
}

pub async fn format_code(path: PathBuf) -> Result<()>{
    let source_code_dir = read_dir(path).await?;
    
    todo!()
}

pub async fn oppend_number(path: PathBuf) -> Result<()>{
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[async_std::test]
    async fn test() {
        let path = PathBuf::from(r"F:\rust_pro\qx");
        read_project(path).await;
    }
}

