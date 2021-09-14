use crate::objects::{AuctionItem, EndedAuctionItem};
#[derive(Serialize, Deserialize)]
pub struct AuctionResult {
    pub auctions: Vec<AuctionItem>,
    #[serde(rename = "totalPages")]
    pub total_pages: i32
}

#[derive(Serialize, Deserialize)]
pub struct EndedAuctionResult {
    pub auctions: Vec<EndedAuctionItem>,
}