use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use reqwest::Client;

use reqwest::Response;

pub trait FileDownloaderTrait{
    async fn download_file(&self,url:&str,file_path:&str)->Result<(),Box<dyn std::error::Error>>;
    async fn create_client(&self)->Client;
    async fn get_responce(&self,client:Client,url:&str)->Result<Response,Box<dyn std::error::Error>>;
    async fn new() -> Self;
}

pub struct Downloader{
    
}
impl FileDownloaderTrait for Downloader{
    async fn new() -> Self
    {
        return Downloader{};
    }
    //ファイルの情報を受け取り書き込み
    async fn download_file(&self,url:&str,file_path:&str)->Result<(),Box<dyn std::error::Error>>
    {
        let client = self.create_client().await;
        let responce = self.get_responce(client,url).await.unwrap().bytes().await.unwrap();
        let mut file = File::create(file_path).await?;
        let _ = file.write_all(&responce).await.unwrap();
        Ok(())
    }

    async fn create_client(&self)->Client
    {
        return Client::new();
    }

    async fn get_responce(&self,client:Client,url:&str)->Result<Response,Box<dyn std::error::Error>>
    {
        let responce = client.get(url).send().await?.error_for_status()?;
        Ok(responce)
    }
}