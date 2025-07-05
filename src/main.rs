use std::{env};

use crate::{file2xml::{File2Xml, File2XmlTrait}, game_file_tool::{GameFileTool, GameFileToolTrait}, tool_download::download_lcf2exe};

mod game_file_tool;
mod tool_download;
mod tool_value;
mod file_downloader;
mod file2xml;
mod logger;
mod error;
mod file_tool;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Loading...");
    println!("Getting Lcf2xml...");
    let mut xml:File2Xml = File2Xml::new().await;
    if xml.check_lcf2xml().await == false{
        download_lcf2exe().await.unwrap();
    }
    let mut game: GameFileTool = GameFileTool{};
    println!("Download complete.");
    println!("Checking Lcf2xml");
    println!("Copying game files");
    if args.len() == 1
    {
        println!("But nobody came.");
        return;
    }
    let copy_game_result = game.copy_game_files(args[1].clone()).await;
    let _ = match copy_game_result{
        Ok(value)=>value,
        Err(e)=>{
            println!("{}",e);
            return;
        },
    };
    println!("Processing lcf2xml");
    println!("Done");
}