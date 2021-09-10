use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct PartialNbt {
	pub i: Vec<PartialNbtElement>,
}

#[derive(Deserialize, Serialize)]
pub struct PartialNbtElement {
	#[serde(rename = "Count")]
	pub count: i8,
	pub tag: PartialTag,
}
//extra?.rarity_upgrades?.value
#[derive(Deserialize, Serialize)]
pub struct PartialTag {
	#[serde(rename = "ExtraAttributes")]
	pub extra_attributes: PartialExtraAttr,
	pub display: DisplayInfo,
}

#[derive(Deserialize, Serialize)]
pub struct PartialExtraAttr {
	#[serde(rename = "rarity_upgrades")]
	pub recombed: Option<i32>, 
	#[serde(rename = "modifier")]
	pub reforge: Option<String>,
	#[serde(rename = "color")]
	pub color: Option<String>,
	#[serde(rename = "runes")]
	pub rune: Option<HashMap<String,i32>>,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "dungeon_item_level")]
	pub stars: Option<i32>,
	#[serde(rename = "hot_potato_count")]
	pub hotpotato: Option<i32>,
    #[serde(rename = "enchantments")]  
	pub enchantments: Option<HashMap<String, i32>>,
}

#[derive(Deserialize, Serialize)]
pub struct DisplayInfo {
	#[serde(rename = "Name")]
	pub name: String,
	#[serde(rename = "Lore")]
	pub lore: Vec<String>,
}