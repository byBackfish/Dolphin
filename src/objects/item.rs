use crate::Result;

use crate::objects::data::PartialNbt;

#[derive(Serialize, Deserialize)]
pub struct ConfiguredAuctionItem {
	pub item: AuctionItem,
	pub nbt: PartialNbt
}

#[derive(Serialize, Deserialize)]
pub struct ConfiguredEndedAuctionItem {
	pub item: EndedAuctionItem,
	pub nbt: PartialNbt
}

#[derive(Serialize, Deserialize)]
pub struct EndedAuctionItem {
	#[serde(rename = "auction_id")]
	pub uuid: String,
   	#[serde(rename = "item_bytes")]
	pub bytes: ItemBytes,
	pub price: i64,
	pub timestamp: i64,
	pub seller: String,
	pub seller_profile: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuctionItem {
   	pub uuid: String,
   	#[serde(rename = "item_bytes")]
	pub bytes: ItemBytes,
	#[serde(rename = "starting_bid")]
	pub price: i64,
	pub start: i64,
	#[serde(rename = "item_lore")]
	pub lore: String,
	pub auctioneer: String,
	pub category: String,
	pub tier: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum ItemBytes {
	T0(ItemBytesT0),
	Data(String),
}

impl Into<String> for ItemBytes {
	fn into(self) -> String {
		match self {
			Self::T0(ibt0) => {
				let ItemBytesT0::Data(x) = ibt0;
				return x;
			}
			Self::Data(x) => x
		}
	}
}

impl Into<Result<Vec<u8>>> for ItemBytes {
	fn into(self) -> Result<Vec<u8>> {
		let b64: String = self.into();
		Ok(base64::decode(&b64).unwrap())
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ItemBytesT0 {
	#[serde(rename = "0")]
	Data(String)
}