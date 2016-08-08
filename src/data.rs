use time::PreciseTime;
use rustc_serialize::json::{self, ToJson, Json};

enum EquipmentType {
	GL200 = 1,
	GL300 = 2,
}

#[derive(RustcEncodable)]
struct SQSPacket {
	//date: PreciseTime,
	//equip_type: EquipmentType,
	raw: String,
}