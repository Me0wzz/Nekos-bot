use crate::types::NTags;

pub async fn get_image(tag: NTags) -> Option<String> {
    let request = reqwest::Client::builder()
        .build()
        .expect("cannot biuld Reqwest client");
    let tag = tag.name().to_lowercase();
    let api_url = format!("https://nekos.life/api/v2/img/{}", tag);
    let response = request
        .get(api_url)
        .send()
        .await
        .expect("cannot get response");
    if response.status() != 200 {
        return None;
    }
    let url = response
        .json::<serde_json::Value>()
        .await
        .expect("cannot parse JSON value")
        .get("url")
        .expect("cannot parse JSON value")
        .as_str()
        .unwrap()
        .to_string();
    Some(url)
}
