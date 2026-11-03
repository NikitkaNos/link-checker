use url::Url;

#[derive(Debug, Clone)] 
pub struct Config{
pub start_url : Url,
pub max_depth: usize,

}
