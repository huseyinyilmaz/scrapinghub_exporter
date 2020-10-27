#[derive(Debug)]
pub struct Result<'a> {
    pub target_results: Vec<TargetResult<'a>>,
}
