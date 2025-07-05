use std::{fs::copy, path::Path, process::Command};

use crate::{error::GetFileError, file_tool::{FileTool, FileToolTrait}};

pub trait GameFileToolTrait{
    async fn copy_game_files(&mut self,game_path:String)->Result<(),GetFileError>;
}

pub struct GameFileTool{
    
}
impl GameFileToolTrait for GameFileTool{
    async fn copy_game_files(&mut self,game_path:String)->Result<(),GetFileError>
    {
        if Path::new(&game_path).is_dir() == false
        {
            return Err(GetFileError::DirectoryError(game_path));
        }
        let mut command_line:String = "".to_string();
        let mut file_tool = FileTool{game_path:game_path.clone()};
        //ファイルの一覧から、lmu,ldb,lmtを取得
        let mut file1 = file_tool.get_lmu_list().await;
        let file2 = match file_tool.get_ldb().await {
            Ok(value)=>value,
            Err(e)=>{
                println!("{}",e);
                return Err(e);
            },
        };
        let file3 = match file_tool.get_lmt().await {
            Ok(value)=>value,
            Err(e)=>{
                println!("{}",e);
                return Err(e);
            },
        };
        let mut files:Vec<String> = vec![];
        files.append(&mut file1);
        files.push(file2);
        files.push(file3);
        let mut command = Command::new("./lcf2xml.exe");
        command.arg("--2k3");

        let mut max_file = 0;
        let mut arg_count = 0;
        let mut executed_count = 0;
        for file in files.iter() 
        {
            let file_copy = copy(file,Path::new(file).file_name().unwrap().to_os_string());
            command.arg(&(Path::new(file).file_name().unwrap().to_str().unwrap()).to_string());
            arg_count += 1;
            if(Path::new(Path::new(Path::new(file).file_name().unwrap())).exists() == false){
                match file_copy {
                    Ok(value) => 
                    {
                        
                    },
                    Err(_) =>
                    {
                        return Err(GetFileError::CopyFileError());
                    }
                };
            }
            if arg_count <= 30
            {
                let _ = command.spawn();
                command = Command::new("./lcf2xml.exe");
                command.arg("--2k3");
                executed_count += 1;
                println!("{}/{}",files.len(),executed_count);
                arg_count = 0;
            }
        }

        

        match command.output() {
            Ok(output) => {
            },
            Err(e) => {
                println!("Command Error: {}", e);
            },
        }

        Ok(())
    }
}