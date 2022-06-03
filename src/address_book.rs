use rusqlite::{
	Connection as DbConnection,
	params,
	Result,
};

pub struct AddressBook {
	pub(crate) connection: DbConnection
}

impl AddressBook {
	pub fn get_phone_number(&self, name: &str) -> Result<String> {
		let mut person_sql = self.connection.prepare("SELECT RowID FROM ABPerson WHERE (First || ' ' || Last)=?1")?;
		let mut person_rows = person_sql.query(params![name])?;
		let person_id = person_rows 
			.next()?
			.unwrap() // TODO: Remove .unwrap()
			.get::<_, i32>(0)?;

		let mut number_sql = self.connection.prepare("SELECT value FROM ABMultiValue WHERE record_id=?1 AND property=?2")?;
		let mut number_rows = number_sql.query(params![person_id, 3_i32])?;
		let mut phone_number = number_rows
			.next()?
			.unwrap() // TODO: Remove .unwrap()
			.get::<_, String>(0)?
			.replace(['(', ')', ' ', '-'], "");
		
		if !phone_number.starts_with('+') {
			phone_number = "+1".to_owned() + &phone_number;
		}

		Ok(phone_number)
	}
}
