#[derive(Clone, Debug)]
pub struct SearchResponse {
    pub titles: Vec<String>,
}
impl SearchResponse {
    pub fn new(titles: Vec<String>) -> SearchResponse {
        SearchResponse { titles }
    }
}
