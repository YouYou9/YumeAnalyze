

use std::fs;

use crate::{error::GetFileError};

pub trait FileToolTrait{
    async fn get_lmu_list(&mut self)->Vec<String>;
    async fn get_ldb(&mut self)->Result<String,GetFileError>;
    async fn get_lmt(&mut self)->Result<String,GetFileError>;
    async fn get_files(&mut self,extension:&str)->Vec<String>;
}
pub struct FileTool{
    pub game_path:String
}
impl FileToolTrait for FileTool{
    async fn get_ldb(&mut self)->Result<String,GetFileError>
    {
        let mut files:Vec<String> = self.get_files("ldb").await;
        if files.len() != 1 {
            return Err(GetFileError::MultipleFileError("ldb".to_string()));
        }
        Ok(files.pop().unwrap())
    }
    async fn get_lmt(&mut self)->Result<String,GetFileError>
    {
        let mut files = self.get_files("lmt").await;
        if files.len() != 1 {
            return Err(GetFileError::MultipleFileError("lmt".to_string()));
        }
        Ok(files.pop().unwrap())
    }
    async fn get_lmu_list(&mut self)->Vec<String>
    {
        let result = self.get_files("lmu").await;
        return result;
    }
    async fn get_files(&mut self,extension:&str)->Vec<String>
    {
        let entries = fs::read_dir(self.game_path.clone());
        let files_str = entries.unwrap().into_iter().filter_map(|entry_result|
        {
            let path = entry_result.unwrap().path();
            path.extension()?
                .to_str()?
                .eq(&extension.to_string())
                .then_some(path.to_str()?.to_string())
        }).collect();
        return files_str;
    }
}