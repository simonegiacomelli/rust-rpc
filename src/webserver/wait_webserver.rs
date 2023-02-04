use std::collections::HashMap;
use std::time::Duration;
use crate::find_port::find_port;
use crate::webserver::HttpResponse;
use crate::webserver::tokio_server::webserver_start;

pub async fn wait_webserver_responsive(url: &str) {
    wait_webserver_responsive_times(url, 300).await.unwrap();
}

pub async fn wait_webserver_responsive_times(url: &str, times: usize) -> Result<(), String> {
    for _ in 0..times {
        let res = reqwest::get(url).await;
        match res {
            Ok(ok) => {
                // print!("{:?} === ", ok);
                // let text = ok.text().await.unwrap();
                // println!("{:?}", text);
                //
                return Ok(());
            }
            Err(err) => {
                // println!("is_connect={}", err.is_connect());
            }
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    Err(format!("timeout waiting for {}", url))
}

#[tokio::test]
async fn test_wait_nothing() {
    let free_port = find_port().unwrap();
    let url = format!("http://127.0.0.1:{}", free_port);
    let result = wait_webserver_responsive_times(&url, 10).await;
    if let Ok(_) = result { panic!("Ci si aspettava un timeout") }
}

#[tokio::test]
async fn test_wait() {
    let port = find_port().unwrap();
    tokio::spawn(async move {
        let string = format!("127.0.0.1:{}", port);
        webserver_start(&string, |req, ctx| -> HttpResponse {
            HttpResponse {
                content: "no content".to_string(),
                content_type: "text/html".to_string(),
                status: 404,
                headers: HashMap::new(),
            }
        }).await.unwrap();
    });
    let url = &format!("http://127.0.0.1:{}", port);
    wait_webserver_responsive(url).await;
    tokio::time::sleep(Duration::new(1, 0)).await;
    reqwest::get(url).await.unwrap().text().await;
    tokio::time::sleep(Duration::new(1, 0)).await;
    // std::thread::sleep(Duration::new(5, 0));
    let result = 2 + 2;
    assert_eq!(result, 4);
}