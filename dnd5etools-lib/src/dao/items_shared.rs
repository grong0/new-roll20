use serde::Deserialize;

#[allow(non_camel_case_types)]
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
	SPC_AAG,
	T,
	#[serde(rename = "T|XPHB")]
	T_XPHB,
	TAH,
	#[serde(rename = "TAH|XPHB")]
	TAH_XPHB,
	#[serde(rename = "TB|XDMG")]
	TB_XDMG,
	TG,
	#[serde(rename = "TG|XDMG")]
	TG_XDMG,
	VEH,
	#[serde(rename = "VEH|XPHB")]
	VEH_XPHB,
	#[serde(rename = "WD|DMG")]
	WD_DMG,
	#[serde(rename = "WD|XDMG")]
	WD_XDMG,
	#[default]
	None,
}

#[allow(non_camel_case_types)]
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

pub type ItemPropertyArray = Vec<ItemProperty>;
