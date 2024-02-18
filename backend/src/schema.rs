use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct PaginationOption {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
#[derive(Deserialize, Debug)]
pub struct ParamOption {
    pub id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateReview {
    pub rating: u8,
    pub content: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateReview {
    pub rating: Option<u8>,
    pub content: Option<String>,
}
