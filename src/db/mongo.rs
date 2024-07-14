use mongodb::{Client, options::ClientOptions};

pub async fn init() -> mongodb::error::Result<Client> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    Client::with_options(client_options)
}
