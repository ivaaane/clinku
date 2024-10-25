use reqwest::Error;
use serde_json::Value;

pub async fn fetch_api(arg: &str, sandbox: bool) -> Result<Value, Error> {
    let path = if sandbox {"sandbox"} else {"words"};

    let url = format!("https://api.linku.la/v1/{}/{}", path, arg);
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    
    if let Some(ok) = response.get("ok") {
        if ok == false {
            println!("{}", response.get("message")
                .and_then(Value::as_str)
                .unwrap_or("Nothing found"));
            std::process::exit(1);
        }
    }

    Ok(response)
}
