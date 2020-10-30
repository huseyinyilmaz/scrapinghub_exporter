use crate::results;
use crate::settings;

pub async fn check_api<'settings>(
    s: &'settings settings::Settings,
) -> results::Result {
    info!("Connecting to scrapinghub api.");
    // empty vector will be assigned from default value.
    // let targets = s.targets.as_ref().unwrap();
    // let target_results: Vec<results::TargetResult> =
    //     join_all(targets.iter().map(process_target)).await;
    // info!("Api requests are complete.");
    let target_results = Vec::new();
    let result = results::Result { target_results };
    debug!("{:?}", result);
    result
}
