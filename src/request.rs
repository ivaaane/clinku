use reqwest::Error;
use serde_json::Value;

pub async fn fetch_api(word: &str, sandbox: bool) -> Result<Value, Error> {
    // if the --sandbox argument is provided, search in the sandbox.
    // else, search common words
    let path = if sandbox {"sandbox"} else {"words"};

    let url = format!("https://api.linku.la/v1/{}/{}", path, word); // api url
    // send request to the url and convert to a json macro
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    
    // comprove success...
    if let Some(ok) = response.get("ok") {
        if ok == false {
            // get and output the error message
            println!("{}", response.get("message")
                .and_then(Value::as_str)
                .unwrap_or("Nothing found")); // this shouldn't happen
            std::process::exit(1); // stop process
        }
    }

    Ok(response)
}
