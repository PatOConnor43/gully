#[derive(Clone, Debug)]
pub struct SearchResponse {
    pub titles: Vec<String>,
}
impl SearchResponse {
    pub fn new(titles: Vec<String>) -> SearchResponse {
        Self { titles }
    }
}

#[derive(Clone, Debug)]
pub struct SearchEntry {
    pub title: String,
    pub length: i64,
    pub url: String,
}
impl SearchEntry {
    pub fn new<T>(title: T, length: i64, url: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            title: title.into(),
            url: url.into(),
            length,
        }
    }
}
