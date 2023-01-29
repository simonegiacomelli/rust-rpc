use crate::proxy::Transport;

pub struct HttpReqwestTransport {
    pub url: String,
}

impl Transport for HttpReqwestTransport {
    fn send(&self, payload: &str) -> String {
        let client = reqwest::blocking::Client::new();
        let string = &self.url;
        let x = payload.to_string();
        let res = client.post(string).body(x).send().unwrap();
        res.text().unwrap()
    }
}
