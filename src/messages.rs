use chrono::{Local, NaiveDateTime, TimeZone};
use crate::{address_book::Contact, Result, TIMESTAMP_OFFSET, DATE_FORMAT_STR};
use rusqlite::{Connection as DbConnection, params};
use std::{fs::{self, File}, io::Write, path::Path};

pub struct Messages {
	pub(crate) connection: DbConnection
}

impl Messages {
	pub fn extract<P: AsRef<Path>>(&self, contact: &Contact, path: P) -> Result<()> {
		let mut file = File::create(path.as_ref())?;

		let mut sql = self.connection.prepare("
			SELECT text, is_from_me, date
			FROM message
			WHERE handle_id=(
				SELECT RowID
				FROM handle
				WHERE id=?1 AND service=?2
			) AND type=?3
		")?;

		let mut rows = sql.query(params![contact.phone_number, "iMessage", 0_i32])?;
		let mut last_timestamp = 0;

		let mut is_empty = true;

		while let Some(row) = rows.next()? {
			let content = row.get::<_, Option<String>>(0)?;
			let is_from_me = row.get::<_, i32>(1)? == 1;
			let raw_timestamp = row.get::<_, i64>(2)? / 1_000_000_000;

			if raw_timestamp - last_timestamp > 7200 { // difference of 2 hours
				let datetime = Local.from_utc_datetime(&NaiveDateTime::from_timestamp(raw_timestamp + TIMESTAMP_OFFSET, 0));

				file.write_all(
					format!("\n      | {} |\n\n", datetime.format(DATE_FORMAT_STR))
						.as_bytes()
				)?;
			}

			file.write_all(
				format!("[{}]: {}\n",
					if is_from_me { "me" } else { &contact.name },
					content.clone().unwrap_or("<unknown>".to_owned()),
				).as_bytes(),
			)?;

			last_timestamp = raw_timestamp;
			is_empty = false;
		}

		if is_empty {
			drop(file);
			fs::remove_file(path.as_ref())?;
		}

		Ok(())
	}
}
