use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
pub struct PartialNbt {
	pub i: Vec<PartialNbtElement>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PartialNbtElement {
	#[serde(rename = "Count")]
	pub count: i8,
	pub tag: PartialTag,
}
//extra?.rarity_upgrades?.value
#[derive(Deserialize, Serialize, Clone)]
pub struct PartialTag {
	#[serde(rename = "ExtraAttributes")]
	pub extra_attributes: PartialExtraAttr,
	pub display: DisplayInfo,
}

#[derive(Deserialize, Serialize, Clone)]
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
	#[serde(rename = "petInfo")]  
	pub pet: Option<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DisplayInfo {
	#[serde(rename = "Name")]
	pub name: String,
	#[serde(rename = "Lore")]
	pub lore: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Pet {
    #[serde(rename = "type")]
    pub pet_type: String,
    #[serde(rename = "tier")]
    pub tier: String,
}
