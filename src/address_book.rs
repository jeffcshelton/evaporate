use rusqlite::{
	Connection as DbConnection,
	params,
	Result,
};

pub struct AddressBook {
	pub(crate) connection: DbConnection
}

impl AddressBook {
	pub fn get_contact(&self, name: &str) -> Result<Contact> {
		let mut sql = self.connection.prepare("
			SELECT value
			FROM ABMultiValue
			WHERE record_id=(
				SELECT RowID
				FROM ABPerson
				WHERE (First || ' ' || Last)=?1
			) AND property=?2"
		)?;

		let mut rows = sql.query(params![name, 3_i32])?;
		let mut phone_number = rows
			.next()?
			.unwrap() // TODO: Remove .unwrap()
			.get::<_, String>(0)?
			.replace(['(', ')', ' ', '-'], "");
		
		if !phone_number.starts_with('+') {
			phone_number = "+1".to_owned() + &phone_number;
		}

		Ok(Contact {
			name: name.to_string(),
			phone_number: phone_number,
		})
	}
}

pub struct Contact {
	pub name: String,
	pub phone_number: String,
}
