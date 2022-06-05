use chrono::{Local, NaiveDateTime, TimeZone};
use crate::{address_book::Contact, Result};
use rusqlite::{Connection as DbConnection, params};
use std::{fs::File, io::Write, path::Path};

const TIMESTAMP_OFFSET: i64 = 978307200; // UNIX timestamp of Jan 1, 2001 @ 00:00 (Apple's choice)

pub struct Messages {
	pub(crate) connection: DbConnection
}

impl Messages {
	pub fn extract<P: AsRef<Path>>(&self, contact: Contact, path: P) -> Result<()> {
		let mut file = File::create(path)?;
			
		let mut handle_sql = self.connection.prepare("SELECT RowID FROM handle WHERE id=?1 AND service=?2")?;
		let mut handle_rows = handle_sql.query(params![contact.phone_number, "iMessage"])?;
		let handle_id: i32 = handle_rows.next()?.unwrap().get(0)?; // TODO: Remove .unwrap()
		
		let mut messages_sql = self.connection.prepare("SELECT text, is_from_me, date FROM message WHERE handle_id=?1 AND type=?2")?;
		let mut message_rows = messages_sql.query(params![handle_id, 0_i32])?;

		let mut last_timestamp = 0;

		while let Some(row) = message_rows.next()? {
			let content = row.get::<_, Option<String>>(0)?;
			let is_from_me = row.get::<_, i32>(1)? == 1;
			let raw_timestamp = row.get::<_, i64>(2)? / 1_000_000_000;

			if raw_timestamp - last_timestamp > 7200 { // difference of 2 hours
				let datetime = Local.from_utc_datetime(&NaiveDateTime::from_timestamp(raw_timestamp + TIMESTAMP_OFFSET, 0));

				file.write_all(
					format!("\n      | {} |\n\n", datetime.format("%A, %B %d, %Y @ %I:%M %p"))
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
		}

		Ok(())
	}

	pub fn extract_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
		// let messages = self.for_contact(&phone_number)?;

		// let mut file = File::create(path)?;

		// for m in 0..messages.len() {
		// 	let msg = &messages[m];
		// 	let last_msg = messages.get(m - 1);

		// 	if (last_msg.is_some() && msg.timestamp - last_msg.unwrap().timestamp > Duration::hours(2)) || (last_msg.is_none()) {
		// 		file.write_all(
		// 			format!("\n      | {} |\n\n", msg.timestamp.format("%A, %B %d, %Y @ %I:%M %p"))
		// 				.as_bytes()
		// 		)?;
		// 	}

		// 	file.write_all(
		// 		format!("[{}]: {}\n",
		// 			if msg.is_from_me { "me" } else { &args.name },
		// 			msg.content.clone().unwrap_or("<unknown>".to_owned()),
		// 		).as_bytes(),
		// 	)?;
		// }

		Ok(())
	}
}
