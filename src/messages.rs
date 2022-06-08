use chrono::{Local, NaiveDateTime, TimeZone, DateTime};
use rusqlite::{Connection as DbConnection, params};
use std::collections::HashMap;

use crate::{
	address_book::Contacts,
	manifest::Manifest,
	Result,
	TIMESTAMP_OFFSET,
};

pub struct Message {
	content: Option<String>,
	is_from_me: bool,
	timestamp: DateTime<Local>
}

pub struct Messages {
	messages: HashMap<String, Vec<Message>>
}

impl Messages {
	pub fn fetch(manifest: &Manifest, contacts: &Contacts) -> Result<Self> {
		let connection = DbConnection::open(manifest.get_path("Library/SMS/sms.db")?)?;
		let mut sql = connection.prepare("
				SELECT
					text,
					is_from_me,
					date / 1000000000
				FROM message
				WHERE handle_id=(
					SELECT RowID
					FROM handle
					WHERE id=?1 AND service IS NOT NULL
				) AND type=?2
			")?;

		let mut messages = HashMap::new();

		for contact in contacts.iter() {
			let mut rows = sql.query(params![contact.phone_number, 0_i32])?;
			let mut conversation: Vec<Message> = Vec::new();

			while let Some(row) = rows.next()? {
				let timestamp = Local.from_utc_datetime(
					&NaiveDateTime::from_timestamp(row.get::<_, i64>(2)? + TIMESTAMP_OFFSET, 0)
				);

				conversation.push(Message {
					content: row.get::<_, Option<String>>(0)?,
					is_from_me: row.get::<_, bool>(1)?,
					timestamp: timestamp,
				});
			}

			messages.insert(contact.name(), conversation);
		}

		Ok(Self { messages: messages })
	}
}
