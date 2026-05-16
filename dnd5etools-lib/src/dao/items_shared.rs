use serde::{
	Deserialize, Deserializer,
	de::{Visitor, value::Error},
};

#[derive(Debug, Default, Deserialize)]
pub enum ItemType {
	DS_DMG,
	DSA_DMG,
	DSA_XDMG,
	DSC,
	DSC_XPHB,
	DSG_DMG,
	DSG_XDMG,
	A,
	A_XPHB,
	AF_DMG,
	AF_XDMG,
	AIR_DMG,
	AIR_XPHB,
	AT,
	AT_XPHB,
	EXP_DMG,
	EXP_XDMG,
	FD,
	FD_XPHB,
	G,
	G_XPHB,
	GS,
	GS_XPHB,
	GV_DMG,
	GV_XDMG,
	HA,
	HA_XPHB,
	INS,
	INS_XPHB,
	LA,
	LA_XPHB,
	M,
	M_XPHB,
	MA,
	MA_XPHB,
	MNT,
	MNT_XPHB,
	OTH,
	P,
	P_XPHB,
	R,
	R_XPHB,
	RD_DMG,
	RD_XDMG,
	RG_DMG,
	RG_XDMG,
	S,
	S_XPHB,
	SC_DMG,
	SC_XPHB,
	SCF,
	SCF_XPHB,
	SHP,
	SHP_XPHB,
	SPC_AAG,
	T,
	T_XPHB,
	TAH,
	TAH_XPHB,
	TB_XDMG,
	TG,
	TG_XDMG,
	VEH,
	VEH_XPHB,
	WD_DMG,
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

#[derive(Debug, Default, Deserialize)]
pub enum ItemProperty {
	TWOH,
	TWOH_XPHB,
	A,
	A_XPHB,
	AF_DMG,
	AF_XDMG,
	BF_DMG,
	BF_XDMG,
	F,
	F_XPHB,
	H,
	H_XPHB,
	L,
	L_XPHB,
	LD,
	LD_XPHB,
	OTH,
	R,
	R_XPHB,
	RLD,
	RLD_XDMG,
	S,
	T,
	T_XPHB,
	V,
	V_XPHB,
	Vst_EGW,
	#[default]
	None,
}

pub fn deserialize_item_property<'de, D>(deserializer: D) -> Result<ItemProperty, D::Error>
where
	D: Deserializer<'de>,
{
	struct ItemPropertyVisitor;

	impl<'de> Visitor<'de> for ItemPropertyVisitor {
		type Value = ItemProperty;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("data");
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			match v {
				"2H" => Ok(ItemProperty::TWOH),
				"2H|XPHB" => Ok(ItemProperty::TWOH_XPHB),
				"A" => Ok(ItemProperty::A),
				"A|XPHB" => Ok(ItemProperty::A_XPHB),
				"AF|DMG" => Ok(ItemProperty::AF_DMG),
				"AF|XDMG" => Ok(ItemProperty::AF_XDMG),
				"BF|DMG" => Ok(ItemProperty::BF_DMG),
				"BF|XDMG" => Ok(ItemProperty::BF_XDMG),
				"F" => Ok(ItemProperty::F),
				"F|XPHB" => Ok(ItemProperty::F_XPHB),
				"H" => Ok(ItemProperty::H),
				"H|XPHB" => Ok(ItemProperty::H_XPHB),
				"L" => Ok(ItemProperty::L),
				"L|XPHB" => Ok(ItemProperty::L_XPHB),
				"LD" => Ok(ItemProperty::LD),
				"LD|XPHB" => Ok(ItemProperty::LD_XPHB),
				"OTH" => Ok(ItemProperty::OTH),
				"R" => Ok(ItemProperty::R),
				"R|XPHB" => Ok(ItemProperty::R_XPHB),
				"RLD" => Ok(ItemProperty::RLD),
				"RLD|XDMG" => Ok(ItemProperty::RLD_XDMG),
				"S" => Ok(ItemProperty::S),
				"T" => Ok(ItemProperty::T),
				"T|XPHB" => Ok(ItemProperty::T_XPHB),
				"V" => Ok(ItemProperty::V),
				"V|XPHB" => Ok(ItemProperty::V_XPHB),
				"Vst|EG" => Ok(ItemProperty::Vst_EGW),
				_ => Err(E::custom("wasn't a valid item property")),
			}
		}
	}
	return deserializer.deserialize_str(ItemPropertyVisitor);
}

pub type ItemPropertyArray = Vec<ItemProperty>;
