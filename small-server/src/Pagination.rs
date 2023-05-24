#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct Pagination {
    pub size: Option<i8>,
    pub page: Option<i64>,
}

#[derive(serde::Serialize)]
pub struct RowCount {
    pub count: Option<i64>,
}

impl Default for Pagination {
    fn default() -> Pagination {
        return Pagination {
            size: Some(10),
            page: Some(0),
        };
    }
}
