use awc::{Client, ClientBuilder, Connector};

// test

#[derive(Default)]
pub struct Crawler {
    pub client: Client,
}
pub struct MyConnector {}

impl Crawler {
    pub fn new() -> Self {
        let client = Client::default();

        Self { client }
    }
}
