use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct TargetResult {
    pub id: String,
    pub size: usize,
    pub duration: u128,
    pub extra_labels: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Result {
    pub target_results: Vec<TargetResult>,
}


impl fmt::Display for TargetResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut labels = self.extra_labels.clone();
        labels.insert("id".to_string(), self.id.to_string());
        let base_labels = labels
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<_>>()
            .join(", ");
        let mut results = Vec::new();
        let duration_stat = format!(
            "scrapinghub_exporter_response_duration_milliseconds{{{} }} {}",
            base_labels, self.duration,
        );
        let size_stat = format!(
            "scrapinghub_exporter_response_response_size_bytes{{{} }} {}",
            base_labels, self.size,
        );
        results.push(duration_stat);
        results.push(size_stat);
        // for query_result in &self.query_results {
        //     let query_stat = format!(
        //         "scrapinghub_exporter_query_count{{{}, query=\"{}\" }} {}",
        //         base_labels,
        //         query_result.query,
        //         query_result.count.unwrap_or(0),
        //     );
        //     results.push(query_stat);
        // }
        write!(f, "{}", results.join("\n"))
    }
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut results = Vec::new();
        for target_result in &self.target_results {
            results.push(format!("{}", target_result));
        }
        write!(f, "{}", results.join("\n"))
    }
}
