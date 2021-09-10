use crate::objects::AuctionItem;
#[derive(Serialize, Deserialize)]
pub struct AuctionResult {
    pub auctions: Vec<AuctionItem>,
    pub totalPages: i32
}