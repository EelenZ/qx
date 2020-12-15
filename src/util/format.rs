use std::error::Error;

use async_std::{path::PathBuf,
                path::Path,
                fs::File,
                fs::read_dir,
                io::{BufReader, BufWriter, prelude::BufReadExt},
};
use futures::StreamExt;

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;

pub async fn read_project(path: PathBuf) -> Result<()> {
    let mut project_root = read_dir(path).await?;
    let mut dir_list: Vec<PathBuf> = Vec::new();
    let mut file_list: Vec<PathBuf> = Vec::new();
    while let Some(res) = project_root.next().await {
        let path = res?.path();
        if path.is_dir().await {
            dir_list.push(path);
        }
        else if path.ends_with(".rs"){
            file_list.push(path);
        }
    }
    
    Ok(())
}

pub async fn list_file(path: PathBuf, dir_list: &mut Vec<PathBuf>, file_list: &mut Vec<PathBuf>) {
    
    todo!()
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

