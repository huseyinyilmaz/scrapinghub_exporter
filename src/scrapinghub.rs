use crate::results;
use crate::settings;
use std::error;

#[derive(Debug)]
struct QueryResponse {
    status: u16,
    size: usize,
}

pub async fn check_api<'settings>(
    s: &'settings settings::Settings,
) -> Result<results::Result, Box<dyn error::Error>> {
    info!("Connecting to scrapinghub api.");
    let client = reqwest::Client::new();
    let url = "https://app.scrapinghub.com/api/jobs/list.jl";
    let req = client.get(url).query(&[
        ("apikey", "a3b65cfed4154b67a96a238221645469"),
        ("project", "38451"),
        ("state", "finished"),
        ("count", "1000"),
    ]);
    let response = req.send().await?;
     // empty vector will be assigned from default value.
    // let targets = s.targets.as_ref().unwrap();
    // let target_results: Vec<results::TargetResult> =
    //     join_all(targets.iter().map(process_target)).await;
    // info!("Api requests are complete.");
    let target_results = Vec::new();

    let result = results::Result { target_results };
    debug!("{:?}", result);
    // result
    Ok(results::Result { target_results: Vec::new()})
}
