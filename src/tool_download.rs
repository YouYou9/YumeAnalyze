use crate::{file_downloader::{Downloader, FileDownloaderTrait}, tool_value::{LCF2XML_EXE, LCF2XML_PATH}};

pub async fn download_lcf2exe()->Result<(),Box<dyn std::error::Error>>{
    let client:Downloader = Downloader::new().await;
    client.download_file(LCF2XML_EXE, LCF2XML_PATH).await.unwrap();
    Ok(())
}
