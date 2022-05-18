use rusqlite::{Connection as DbConnection, params, Result};
use crate::Manifest;

pub struct AddressBook {
	connection: DbConnection
}

impl AddressBook {
	fn open(manifest: &Manifest) -> Result<Self> {
		let address_book_path = manifest.get_path("Library/AddressBook/AddressBook.sqlitedb")?;

		Ok(Self {
			connection: DbConnection::open(address_book_path)?
		})
	}

	pub fn get_phone_number(&self, name: &str) -> Result<String> {
		let name = name.split(' ').collect::<Vec<&str>>();
		let first_name = name.first();
		let last_name = name.last();

		let mut person_sql = self.connection.prepare("SELECT RowID FROM ABPerson WHERE First=?1 AND Last=?2")?;
		let mut person_rows = person_sql.query(params![first_name, last_name])?;
		let person_id: i32 = person_rows.next()?.unwrap().get(0)?; // TODO: Remove .unwrap()

		let mut number_sql = self.connection.prepare("SELECT value FROM ABMultiValue WHERE record_id=?1 AND property=?2")?;
		let mut number_rows = number_sql.query(params![person_id, 3_i32])?;
		let phone_number: String = number_rows.next()?.unwrap().get(0)?; // TODO: Remove .unwrap()

		Ok(phone_number)
	}
}
