use std::str::FromStr;

pub enum EquipmentType {
	Gl200,
	Gl300,
}

impl FromStr for EquipmentType {
	type Err = ();

	fn from_str(s: &str) -> Result<EquipmentType, ()> {
		match s {
			"Gl200" => Ok(EquipmentType::Gl200),
			"Gl300" => Ok(EquipmentType::Gl300),
			_ => Err(()),
		}
	}
}