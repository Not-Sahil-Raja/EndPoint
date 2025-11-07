use serde::Serialize;
use tauri::command;
use tauri_plugin_http::reqwest;

#[derive(Serialize)]
pub struct HealthResult {
    pub url: String,
    pub status: String,
    pub response_time: u128,
}

#[command]
pub async fn check_health(url: String) -> Result<HealthResult, String> {
    let start = std::time::Instant::now();
    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await;
    let elapsed = start.elapsed().as_millis();

    match res {
        Ok(response) => {
            let status = response.status().to_string();

            Ok(HealthResult {
                url,
                status,
                response_time: elapsed,
            })
        }
        Err(err) => Err(err.to_string()),
    }
}
