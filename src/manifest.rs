use {
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
	pub fn open(backup_path: &Path) -> Result<Self> {
		let manifest_path = backup_path.join("Manifest.db");

		Ok(Self {
			local_backup_path: backup_path.to_owned(),
			connection: DbConnection::open(manifest_path)?,
		})
	}

	pub fn get_path(&self, device_relative_path: &str) -> Result<PathBuf> {
		let mut sql = self.connection.prepare("SELECT fileID FROM Files WHERE relativePath=?1")?;

		let mut file_hash = sql.query_row(
			params![device_relative_path],
			|row| Ok(row.get::<_, String>(0)?)
		)?;

		let full_path = self.local_backup_path
			.join(&file_hash[..2])
			.join(file_hash);

		Ok(full_path)
	}
}
