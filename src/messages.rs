use rusqlite::{Connection as DbConnection, params};

use chrono::{
	Local,
	NaiveDateTime,
	TimeZone,
	DateTime,
	Duration,
};

use crate::{
	constants::TIMESTAMP_OFFSET,
	manifest::Manifest,
	Result,
};

use std::{
	collections::HashMap,
	fs::{self, File},
	io::Write,
	path::Path,
};

pub struct Message {
	content: Option<String>,
	is_from_me: bool,
	timestamp: DateTime<Local>
}

fn fetch(manifest: &Manifest) -> Result<HashMap<String, Vec<Message>>> {
	let messages_path = manifest.get_path("Library/SMS/sms.db")?;
	let contacts_path = manifest.get_path("Library/AddressBook/AddressBook.sqlitedb")?;

	let connection = DbConnection::open(messages_path)?;
	connection.execute("ATTACH DATABASE ?1 AS AddressBook", params![contacts_path.to_string_lossy()])?;

	let mut sql = connection.prepare("
		SELECT
			Message.text,
			Message.is_from_me,
			Message.date / 1000000000,
			Contact.First || COALESCE(' ' || Contact.Last, '')
		FROM Message
		INNER JOIN AddressBook.ABMultiValue AS PhoneNumber ON
			PhoneNumber.property = 3
			AND handle.id = REPLACE(REPLACE(REPLACE(REPLACE(
				CASE WHEN PhoneNumber.value NOT LIKE '+%'
					THEN '+1' || PhoneNumber.value
					ELSE PhoneNumber.value
				END,
			'(', ''), ')', ''), ' ', ''), '-', '')
		INNER JOIN AddressBook.ABPerson AS Contact ON
			Contact.RowID = PhoneNumber.record_id
		INNER JOIN handle ON
			Message.handle_id = handle.RowID
			AND handle.service IS NOT NULL
		WHERE
			Message.type = 0
			AND Contact.First IS NOT NULL
		ORDER BY Message.date ASC
	")?;

	let mut messages = HashMap::<String, Vec<Message>>::new();
	let mut rows = sql.query([])?;

	while let Some(row) = rows.next()? {
		let name: String = row.get(3)?;
		let timestamp = Local.from_utc_datetime(
			&NaiveDateTime::from_timestamp_opt(row.get::<_, i64>(2)? + TIMESTAMP_OFFSET, 0)
				.expect("! invalid timestamp found in database !")
		);

		let message = Message {
			content: row.get(0)?,
			is_from_me: row.get(1)?,
			timestamp: timestamp,
		};

		if let Some(conversation) = messages.get_mut(&name) {
			conversation.push(message);
		} else {
			messages.insert(name, vec![message]);
		}
	}

	Ok(messages)
}

pub fn extract_to<P: AsRef<Path>>(path: P, manifest: &Manifest) -> Result<()> {
	let path = path.as_ref();
	fs::create_dir(path)?;

	for (name, conversation) in fetch(manifest)? {
		if conversation.is_empty() {
			continue;
		}

		let mut sanitized_name = name
			.trim_matches(['.', ' '].as_slice())
			.replace(['<', '>', ':', '"', '/', '\\', '|', '?', '*'], "-")
			.replace(|c: char| c.is_ascii_control(), "");

		if sanitized_name.is_empty() {
			sanitized_name = "Unnamed".to_owned();
		}

		let mut export_path = path.join(&sanitized_name).with_extension("txt");
		let mut repeats = 0;

		// enumerate the contacts if there are multiple with the same name
		while export_path.exists() {
			repeats += 1;
			export_path.set_file_name(sanitized_name.clone() + " " + &repeats.to_string());
		}

		let mut file = File::create(export_path)?;
		let mut last_timestamp = Local.timestamp_opt(0, 0).unwrap();

		for message in conversation {
			// if there was more than two hours between texts,
			// write a new timestamp preceding the message content
			if message.timestamp - last_timestamp > Duration::hours(2) {
				let timestamp = message.timestamp.format("%A, %B %d, %Y @ %I:%M %p");

				write!(&mut file, "\n      | {timestamp} |\n\n")?;
			}

			// write message to text file
			let sender = if message.is_from_me { "me" } else { &name };
			let content = message.content
				.as_deref()
				.unwrap_or("<unknown>");

			writeln!(&mut file, "[{sender}]: {content}")?;

			last_timestamp = message.timestamp;
		}
	}

	Ok(())
}
