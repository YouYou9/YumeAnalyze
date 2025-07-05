use std::{error::Error};

use sha2::{Digest, Sha256};
use tokio::{fs::{try_exists, File}, io::{AsyncReadExt, AsyncWriteExt}};
use crate::{logger::log_error, tool_value::{LCF2XML_HASH_PATH, LCF2XML_PATH}};


pub trait File2XmlTrait{
    async fn check_hash(&mut self) -> bool;
    async fn check_exists(&mut self) -> bool;
    async fn new() -> Self;
    async fn check_lcf2xml(&mut self) ->bool;
    #[allow(dead_code)]
    async fn get_hash(&mut self) -> bool;
}

pub struct File2Xml{
    #[allow(dead_code)]
    path:String,
}
//Opening a file with Vec<v8>
pub async fn open_file(path:&str)->Result <Vec<u8>,Box<dyn std::error::Error>>
{
    let file_h = File::open(path).await;
    let mut file:File = match file_h {
        Ok(value) => value,
        Err(e) =>
        {
            return Err(From::from(e));
        }
    };
    let mut buffer:Vec<u8> = vec![];
    let _ = file.read_to_end(&mut buffer).await;
    Ok(buffer)
}
impl File2XmlTrait for File2Xml{
    async fn new() -> Self
    {
        return File2Xml{path:".".to_string()};
    }
    //A routine for checking lcf2xml, returns a boolean value.
    async fn check_lcf2xml(&mut self) ->bool{
        if self.check_exists().await == true{
            if self.check_hash().await == true{
                return true;
            }
        }else{
            log_error("lcf2xml did not exist.1");
        }
        return false;
    }
    //Check file existence
    async fn check_exists(&mut self) -> bool
    {
        return try_exists(LCF2XML_PATH).await.unwrap();
    }
    //Checking the hash
    async fn check_hash(&mut self) -> bool
    {
        //open exe file
        let file_exe: Result<Vec<u8>, Box<dyn Error>> = open_file(LCF2XML_PATH).await;
        
        let file:Vec<u8> = match file_exe {
            Ok(value) => value,
            Err(_) =>
            {
                log_error("lcf2xml did not exist.2");
                return false;
            }
        };
        //Calculate the SHA of an exe
        let sha = Sha256::digest(file);
        //Get the hash of a file
        let file_hash = open_file(LCF2XML_HASH_PATH).await;
        
        let file:Vec<u8> = match file_hash {
            Ok(value) => value,
            Err(_) =>
            {
                log_error("The hash file did not exist.");
                return false;
            }
        };
        //If the hash values ​​are the same
        if file == sha.to_vec(){
            return true;
        }

        log_error("The lcf2xml hash was invalid.");
        return false;
    }
    //Get hash file
    async fn get_hash(&mut self) -> bool
    {
        let file_exe: Result<Vec<u8>, Box<dyn Error>> = open_file(LCF2XML_PATH).await;
        
        let file:Vec<u8> = match file_exe {
            Ok(value) => value,
            Err(_) =>
            {
                log_error("lcf2xml did not exist.2");
                return false;
            }
        };

        let sha = Sha256::digest(file);
        
        let mut hash_file = File::create(LCF2XML_HASH_PATH).await.unwrap();
        _ = hash_file.write_all(&sha.to_vec()).await.unwrap();
        return try_exists(LCF2XML_PATH).await.unwrap();
    }
}