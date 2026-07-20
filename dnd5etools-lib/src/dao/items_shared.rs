use serde::{Deserialize, Deserializer, de::Visitor};

#[derive(Debug, Default, Deserialize)]
pub enum ItemType {
	#[serde(rename = "$|DMG")]
	DS_DMG,
	#[serde(rename = "$A|DMG")]
	DSA_DMG,
	#[serde(rename = "$A|XDMG")]
	DSA_XDMG,
	#[serde(rename = "$C")]
	DSC,
	#[serde(rename = "$C|XPHB")]
	DSC_XPHB,
	#[serde(rename = "$G|DMG")]
	DSG_DMG,
	#[serde(rename = "$G|XDMG")]
	DSG_XDMG,
	A,
	#[serde(rename = "A|XPHB")]
	A_XPHB,
	#[serde(rename = "AF|DMG")]
	AF_DMG,
	#[serde(rename = "AF|XDMG")]
	AF_XDMG,
	#[serde(rename = "AIR|DMG")]
	AIR_DMG,
	#[serde(rename = "AIR|XPHB")]
	AIR_XPHB,
	AT,
	#[serde(rename = "AT|XPHB")]
	AT_XPHB,
	#[serde(rename = "EXP|DMG")]
	EXP_DMG,
	#[serde(rename = "EXP|XDMG")]
	EXP_XDMG,
	FD,
	#[serde(rename = "FD|XPHB")]
	FD_XPHB,
	G,
	#[serde(rename = "G|XPHB")]
	G_XPHB,
	GS,
	#[serde(rename = "GS|XPHB")]
	GS_XPHB,
	#[serde(rename = "GV|DMG")]
	GV_DMG,
	#[serde(rename = "GV|XDMG")]
	GV_XDMG,
	HA,
	#[serde(rename = "HA|XPHB")]
	HA_XPHB,
	INS,
	#[serde(rename = "INS|XPHB")]
	INS_XPHB,
	LA,
	#[serde(rename = "LA|XPHB")]
	LA_XPHB,
	M,
	#[serde(rename = "M|XPHB")]
	M_XPHB,
	MA,
	#[serde(rename = "MA|XPHB")]
	MA_XPHB,
	MNT,
	#[serde(rename = "MNT|XPHB")]
	MNT_XPHB,
	OTH,
	P,
	#[serde(rename = "P|XPHB")]
	P_XPHB,
	R,
	#[serde(rename = "R|XPHB")]
	R_XPHB,
	#[serde(rename = "RD|DMG")]
	RD_DMG,
	#[serde(rename = "RD|XDMG")]
	RD_XDMG,
	#[serde(rename = "RG|DMG")]
	RG_DMG,
	#[serde(rename = "RG|XDMG")]
	RG_XDMG,
	S,
	#[serde(rename = "S|XPHB")]
	S_XPHB,
	#[serde(rename = "SC|DMG")]
	SC_DMG,
	#[serde(rename = "SC|XPHB")]
	SC_XPHB,
	SCF,
	#[serde(rename = "SCF|XPHB")]
	SCF_XPHB,
	SHP,
	#[serde(rename = "SHP|XPHB")]
	SHP_XPHB,
	#[serde(rename = "SPC|AAG")]
	SPC_AAG,	T,
	#[serde(rename = "T|XPHB")]
	T_XPHB,	TAH,
	#[serde(rename = "TAH|XPHB")]
	TAH_XPHB,
	#[serde(rename = "TB|XDMG")]
	TB_XDMG,	TG,
	#[serde(rename = "TG|XDMG")]
	TG_XDMG,	VEH,
	#[serde(rename = "VEH|XPHB")]
	VEH_XPHB,
	#[serde(rename = "WD|DMG")]
	WD_DMG,
	#[serde(rename = "WD|XDMG")]
	WD_XDMG,
	#[default]
	None,
}

pub fn deserialize_item_type<'de, D>(deserializer: D) -> Result<ItemType, D::Error>
where
	D: Deserializer<'de>,
{
	struct ItemTypeVisitor;

	impl<'de> Visitor<'de> for ItemTypeVisitor {
		type Value = ItemType;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("data");
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			match v {
				"$|DMG" => Ok(ItemType::DS_DMG),
				"$A|DMG" => Ok(ItemType::DSA_DMG),
				"$A|XDMG" => Ok(ItemType::DSA_XDMG),
				"$C" => Ok(ItemType::DSC),
				"$C|XPHB" => Ok(ItemType::DSC_XPHB),
				"$G|DMG" => Ok(ItemType::DSG_DMG),
				"$G|XDMG" => Ok(ItemType::DSG_XDMG),
				"A" => Ok(ItemType::A),
				"A|XPHB" => Ok(ItemType::A_XPHB),
				"AF|DMG" => Ok(ItemType::AF_DMG),
				"AF|XDMG" => Ok(ItemType::AF_XDMG),
				"AIR|DMG" => Ok(ItemType::AIR_DMG),
				"AIR|XPHB" => Ok(ItemType::AIR_XPHB),
				"AT" => Ok(ItemType::AT),
				"AT|XPHB" => Ok(ItemType::AT_XPHB),
				"EXP|DMG" => Ok(ItemType::EXP_DMG),
				"EXP|XDMG" => Ok(ItemType::EXP_XDMG),
				"FD" => Ok(ItemType::FD),
				"FD|XPHB" => Ok(ItemType::FD_XPHB),
				"G" => Ok(ItemType::G),
				"G|XPHB" => Ok(ItemType::G_XPHB),
				"GS" => Ok(ItemType::GS),
				"GS|XPHB" => Ok(ItemType::GS_XPHB),
				"GV|DMG" => Ok(ItemType::GV_DMG),
				"GV|XDMG" => Ok(ItemType::GV_XDMG),
				"HA" => Ok(ItemType::HA),
				"HA|XPHB" => Ok(ItemType::HA_XPHB),
				"INS" => Ok(ItemType::INS),
				"INS|XPHB" => Ok(ItemType::INS_XPHB),
				"LA" => Ok(ItemType::LA),
				"LA|XPHB" => Ok(ItemType::LA_XPHB),
				"M" => Ok(ItemType::M),
				"M|XPHB" => Ok(ItemType::M_XPHB),
				"MA" => Ok(ItemType::MA),
				"MA|XPHB" => Ok(ItemType::MA_XPHB),
				"MNT" => Ok(ItemType::MNT),
				"MNT|XPHB" => Ok(ItemType::MNT_XPHB),
				"OTH" => Ok(ItemType::OTH),
				"P" => Ok(ItemType::P),
				"P|XPHB" => Ok(ItemType::P_XPHB),
				"R" => Ok(ItemType::R),
				"R|XPHB" => Ok(ItemType::R_XPHB),
				"RD|DMG" => Ok(ItemType::RD_DMG),
				"RD|XDMG" => Ok(ItemType::RD_XDMG),
				"RG|DMG" => Ok(ItemType::RG_DMG),
				"RG|XDMG" => Ok(ItemType::RG_XDMG),
				"S" => Ok(ItemType::S),
				"S|XPHB" => Ok(ItemType::S_XPHB),
				"SC|DMG" => Ok(ItemType::SC_DMG),
				"SC|XPHB" => Ok(ItemType::SC_XPHB),
				"SCF" => Ok(ItemType::SCF),
				"SCF|XPHB" => Ok(ItemType::SCF_XPHB),
				"SHP" => Ok(ItemType::SHP),
				"SHP|XPHB" => Ok(ItemType::SHP_XPHB),
				"SPC|AAG" => Ok(ItemType::SPC_AAG),
				"T" => Ok(ItemType::T),
				"T|XPHB" => Ok(ItemType::T_XPHB),
				"TAH" => Ok(ItemType::TAH),
				"TAH|XPHB" => Ok(ItemType::TAH_XPHB),
				"TB|XDMG" => Ok(ItemType::TB_XDMG),
				"TG" => Ok(ItemType::TG),
				"TG|XDMG" => Ok(ItemType::TG_XDMG),
				"VEH" => Ok(ItemType::VEH),
				"VEH|XPHB" => Ok(ItemType::VEH_XPHB),
				"WD|DMG" => Ok(ItemType::WD_DMG),
				"WD|XDMG" => Ok(ItemType::WD_XDMG),
				_ => Err(E::custom("wasn't a valid item type")),
			}
		}
	}

	return deserializer.deserialize_str(ItemTypeVisitor);
}

pub fn deserialize_item_type_array<'de, D>(deserializer: D) -> Result<Vec<ItemType>, D::Error>
where
	D: Deserializer<'de>,
{
	struct ItemTypeArrayVisitor;

	impl<'de> Visitor<'de> for ItemTypeArrayVisitor {
		type Value = Vec<ItemType>;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("data");
		}

		fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::SeqAccess<'de>,
		{
			let mut values = vec![];
			let element = seq.next_element()?;
			while element.is_some() {
				let element_value: &str = element.unwrap();
				let value = match element_value {
					"$|DMG" => ItemType::DS_DMG,
					"$A|DMG" => ItemType::DSA_DMG,
					"$A|XDMG" => ItemType::DSA_XDMG,
					"$C" => ItemType::DSC,
					"$C|XPHB" => ItemType::DSC_XPHB,
					"$G|DMG" => ItemType::DSG_DMG,
					"$G|XDMG" => ItemType::DSG_XDMG,
					"A" => ItemType::A,
					"A|XPHB" => ItemType::A_XPHB,
					"AF|DMG" => ItemType::AF_DMG,
					"AF|XDMG" => ItemType::AF_XDMG,
					"AIR|DMG" => ItemType::AIR_DMG,
					"AIR|XPHB" => ItemType::AIR_XPHB,
					"AT" => ItemType::AT,
					"AT|XPHB" => ItemType::AT_XPHB,
					"EXP|DMG" => ItemType::EXP_DMG,
					"EXP|XDMG" => ItemType::EXP_XDMG,
					"FD" => ItemType::FD,
					"FD|XPHB" => ItemType::FD_XPHB,
					"G" => ItemType::G,
					"G|XPHB" => ItemType::G_XPHB,
					"GS" => ItemType::GS,
					"GS|XPHB" => ItemType::GS_XPHB,
					"GV|DMG" => ItemType::GV_DMG,
					"GV|XDMG" => ItemType::GV_XDMG,
					"HA" => ItemType::HA,
					"HA|XPHB" => ItemType::HA_XPHB,
					"INS" => ItemType::INS,
					"INS|XPHB" => ItemType::INS_XPHB,
					"LA" => ItemType::LA,
					"LA|XPHB" => ItemType::LA_XPHB,
					"M" => ItemType::M,
					"M|XPHB" => ItemType::M_XPHB,
					"MA" => ItemType::MA,
					"MA|XPHB" => ItemType::MA_XPHB,
					"MNT" => ItemType::MNT,
					"MNT|XPHB" => ItemType::MNT_XPHB,
					"OTH" => ItemType::OTH,
					"P" => ItemType::P,
					"P|XPHB" => ItemType::P_XPHB,
					"R" => ItemType::R,
					"R|XPHB" => ItemType::R_XPHB,
					"RD|DMG" => ItemType::RD_DMG,
					"RD|XDMG" => ItemType::RD_XDMG,
					"RG|DMG" => ItemType::RG_DMG,
					"RG|XDMG" => ItemType::RG_XDMG,
					"S" => ItemType::S,
					"S|XPHB" => ItemType::S_XPHB,
					"SC|DMG" => ItemType::SC_DMG,
					"SC|XPHB" => ItemType::SC_XPHB,
					"SCF" => ItemType::SCF,
					"SCF|XPHB" => ItemType::SCF_XPHB,
					"SHP" => ItemType::SHP,
					"SHP|XPHB" => ItemType::SHP_XPHB,
					"SPC|AAG" => ItemType::SPC_AAG,
					"T" => ItemType::T,
					"T|XPHB" => ItemType::T_XPHB,
					"TAH" => ItemType::TAH,
					"TAH|XPHB" => ItemType::TAH_XPHB,
					"TB|XDMG" => ItemType::TB_XDMG,
					"TG" => ItemType::TG,
					"TG|XDMG" => ItemType::TG_XDMG,
					"VEH" => ItemType::VEH,
					"VEH|XPHB" => ItemType::VEH_XPHB,
					"WD|DMG" => ItemType::WD_DMG,
					"WD|XDMG" => ItemType::WD_XDMG,
					_ => ItemType::None,
				};
				values.push(value);
			}
			Ok(values)
		}
	}

	return deserializer.deserialize_seq(ItemTypeArrayVisitor);
}

#[derive(Debug, Default, Deserialize)]
pub enum ItemProperty {
	#[serde(rename = "2H")]
	TWOH,
	#[serde(rename = "2H|XPHB")]
	TWOH_XPHB,
	A,
	#[serde(rename = "A|XPHB")]
	A_XPHB,
	#[serde(rename = "AF|DMG")]
	AF_DMG,
	#[serde(rename = "AF|XDMG")]
	AF_XDMG,
	#[serde(rename = "BF|DMG")]
	BF_DMG,
	#[serde(rename = "BF|XDMG")]
	BF_XDMG,
	F,
	#[serde(rename = "F|XPHB")]
	F_XPHB,
	H,
	#[serde(rename = "H|XPHB")]
	H_XPHB,
	L,
	#[serde(rename = "L|XPHB")]
	L_XPHB,
	LD,
	#[serde(rename = "LD|XPHB")]
	LD_XPHB,
	OTH,
	R,
	#[serde(rename = "R|XPHB")]
	R_XPHB,
	RLD,
	#[serde(rename = "RLD|XDMG")]
	RLD_XDMG,
	S,
	T,
	#[serde(rename = "T|XPHB")]
	T_XPHB,
	V,
	#[serde(rename = "V|XPHB")]
	V_XPHB,
	#[serde(rename = "Vst|EG")]
	Vst_EGW,
	#[default]
	None,
}

pub fn deserialize_item_property_array<'de, D>(deserializer: D) -> Result<ItemPropertyArray, D::Error>
where
	D: Deserializer<'de>,
{
	struct ItemPropertyArrayVisitor;

	impl<'de> Visitor<'de> for ItemPropertyArrayVisitor {
		type Value = ItemPropertyArray;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("data");
		}

		fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::SeqAccess<'de>,
		{
			let mut values = vec![];
			let element = seq.next_element()?;
			while element.is_some() {
				let element_value: &str = element.unwrap();
				let value = match element_value {
					"2H" => ItemProperty::TWOH,
					"2H|XPHB" => ItemProperty::TWOH_XPHB,
					"A" => ItemProperty::A,
					"A|XPHB" => ItemProperty::A_XPHB,
					"AF|DMG" => ItemProperty::AF_DMG,
					"AF|XDMG" => ItemProperty::AF_XDMG,
					"BF|DMG" => ItemProperty::BF_DMG,
					"BF|XDMG" => ItemProperty::BF_XDMG,
					"F" => ItemProperty::F,
					"F|XPHB" => ItemProperty::F_XPHB,
					"H" => ItemProperty::H,
					"H|XPHB" => ItemProperty::H_XPHB,
					"L" => ItemProperty::L,
					"L|XPHB" => ItemProperty::L_XPHB,
					"LD" => ItemProperty::LD,
					"LD|XPHB" => ItemProperty::LD_XPHB,
					"OTH" => ItemProperty::OTH,
					"R" => ItemProperty::R,
					"R|XPHB" => ItemProperty::R_XPHB,
					"RLD" => ItemProperty::RLD,
					"RLD|XDMG" => ItemProperty::RLD_XDMG,
					"S" => ItemProperty::S,
					"T" => ItemProperty::T,
					"T|XPHB" => ItemProperty::T_XPHB,
					"V" => ItemProperty::V,
					"V|XPHB" => ItemProperty::V_XPHB,
					"Vst|EG" => ItemProperty::Vst_EGW,
					_ => ItemProperty::None,
				};
				if matches!(value, ItemProperty::None) {
					return Err(serde::de::Error::custom(""));
				}
				values.push(value);
			}
			Ok(values)
		}
	}
	return deserializer.deserialize_seq(ItemPropertyArrayVisitor);
}

pub type ItemPropertyArray = Vec<ItemProperty>;
