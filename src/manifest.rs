use {
	crate::{
		address_book::AddressBook,
		messages::Messages,
	},
	rusqlite::{
		Connection as DbConnection,
		params,
		Result,
	},
	std::path::{
		Path,
		PathBuf,
	},
};

pub struct Manifest {
	local_backup_path: PathBuf,
	connection: DbConnection,
}

impl Manifest {
	pub fn address_book(&self) -> Result<AddressBook> {
		let address_book_path = self.get_path("Library/AddressBook/AddressBook.sqlitedb")?;

		Ok(AddressBook {
			connection: DbConnection::open(address_book_path)?
		})
	}

	pub fn messages(&self) -> Result<Messages> {
		let messages_path = self.get_path("Library/SMS/sms.db")?;

		Ok(Messages {
			connection: DbConnection::open(messages_path)?
		})
	}

	pub fn open(backup_path: &Path) -> Result<Self> {
		let manifest_path = backup_path.join("Manifest.db");

		Ok(Self {
			local_backup_path: backup_path.to_owned(),
			connection: DbConnection::open(manifest_path)?,
		})
	}

	pub fn get_path(&self, device_relative_path: &str) -> Result<PathBuf> {
		let mut sql = self.connection.prepare("SELECT fileID FROM Files WHERE relativePath=?1")?;
		let mut rows = sql.query(params![device_relative_path])?;

		let file_hash: String = rows.next()?.unwrap().get(0)?; // TODO: Remove .unwrap()
		let full_path = self.local_backup_path
			.join(&file_hash[..2])
			.join(file_hash);

		Ok(full_path)
	}
}
