use std::time::Duration;

pub async fn wait_webserver_responsive(url: &str) {
    for _ in 0..300 {
        let res = reqwest::get(url).await;
        println!("{:?}", res);
        match res {
            Ok(ok) => { return; }
            Err(err) => {
                println!("is_connect={}", err.is_connect());
            }
        }

        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    panic!("timeout waiting for {}", url);
}
