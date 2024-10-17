use reqwest::Error;
use serde_json::Value;

pub async fn fetch_api(arg: &str) -> Result<Value, Error> {
    let url = format!("https://api.linku.la/v1/words/{}", arg);
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    Ok(response)
}
