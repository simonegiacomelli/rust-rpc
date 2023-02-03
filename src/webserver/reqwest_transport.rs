use crate::proxy::Transport;
use async_trait::async_trait;

pub struct HttpReqwestTransport {
    pub url: String,
}

#[async_trait]
impl Transport for HttpReqwestTransport {
    async fn send(&self, payload: &str) -> String {
        let client = reqwest::Client::new();
        let string = &self.url;
        let x = payload.to_string();
        let res = client.post(string).body(x).send().await.unwrap();
        res.text().await.unwrap()
    }
}
