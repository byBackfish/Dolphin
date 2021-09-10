use crate::Result;

use crate::objects::data::PartialNbt;

#[derive(Serialize, Deserialize)]
pub struct ConfiguredAuctionItem {
	pub item: AuctionItem,
	pub nbt: PartialNbt
}

#[derive(Serialize, Deserialize)]
pub struct AuctionItem {
   pub uuid: String,
   	#[serde(rename = "item_bytes")]
	pub bytes: ItemBytes,
	#[serde(rename = "starting_bid")]
	pub price: i64,
	
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