use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Review {
    pub id: uuid::Uuid,
    pub content: String,
    pub rating: u8,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ReviewResponse {
    pub status: u16,
    pub review: Review,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ReviewListResponse {
    pub status: u16,
    pub reviews: Vec<Review>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
}
