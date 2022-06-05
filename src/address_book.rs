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
			) AND property=?2
		")?;

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

	pub fn get_all(&self) -> Result<Vec<Contact>> {
		let mut sql = self.connection.prepare("
			SELECT ABPerson.First || COALESCE(' ' || ABPerson.Last, ''), ABMultiValue.value
			FROM ABPerson
			INNER JOIN ABMultiValue
			ON ABPerson.RowID=ABMultiValue.record_id AND property=?1
		")?;

		let mut rows = sql.query(params![3_i32])?;
		let mut contacts = Vec::new();

		while let Some(row) = rows.next()? {
			let name = row.get::<_, Option<String>>(0)?;
			let phone_number = row.get::<_, String>(1)?;

			if let Some(name) = name {
				contacts.push(Contact {
					name: name,
					phone_number: phone_number,
				});
			}
		}

		Ok(contacts)
	}
}

pub struct Contact {
	pub name: String,
	pub phone_number: String,
}
