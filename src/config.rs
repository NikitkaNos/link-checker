use url::Url;

#[derive(Debug, Clone)]
pub struct Config {
    pub start_url: Url,
    pub max_depth: usize,
    pub max_pages: usize,
    pub timeout_secs: u64,
    pub concurrency: usize,
}
