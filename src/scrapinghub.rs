pub async fn check_api<'result, 'settings: 'result>(
    s: &'settings settings::Settings,
) -> results::Result<'result> {
    info!("Connecting to scrapinghub api.");
    // empty vector will be assigned from default value.
    let targets = s.targets.as_ref().unwrap();
    let target_results: Vec<results::TargetResult> =
        join_all(targets.iter().map(process_target)).await;
    info!("Api requests are complete.");
    let result = results::Result { target_results };
    debug!("{:?}", result);
    result
}
