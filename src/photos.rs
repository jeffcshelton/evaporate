use rusqlite::Connection as DbConnection;

use crate::{
	Manifest,
	Result,
};

use std::{
	fs,
	path::{Path, PathBuf},
};

pub struct Photo {
	file_name: String,
	path: PathBuf,
}

pub struct Photos {
	photos: Vec<Photo>
}

impl Photos {
	pub fn fetch(manifest: &Manifest) -> Result<Self> {
		let connection = DbConnection::open(manifest.get_path("Media/PhotoData/Photos.sqlite")?)?;
		let mut sql = connection.prepare("
			SELECT
				ZFilename,
				'Media/' || ZDirectory || '/' || ZFilename
			FROM ZAsset
			WHERE ZDirectory LIKE 'DCIM/%' AND ZFilename IS NOT NULL
		")?;

		let mut rows = sql.query([])?;
		let mut photos = Vec::new();

		while let Some(row) = rows.next()? {
			photos.push(Photo {
				file_name: row.get(0)?,
				path: manifest.get_path(&row.get::<_, String>(1)?)?
			});
		}

		Ok(Self { photos: photos })
	}

	pub fn extract_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
		let path = path.as_ref();
		fs::create_dir(path)?;

		for photo in &self.photos {
			fs::copy(&photo.path, path.join(&photo.file_name))?;
		}

		Ok(())
	}
}
