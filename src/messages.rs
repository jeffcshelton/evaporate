use {
	chrono::{
		DateTime,
		NaiveDateTime,
		Local,
		TimeZone,
	},
	rusqlite::{
		Connection as DbConnection,
		params,
		Result,
	},
};

const TIMESTAMP_OFFSET: i64 = 978307200; // UNIX timestamp of Jan 1, 2001 @ 00:00 (Apple's choice)

pub struct Messages {
	pub(crate) connection: DbConnection
}

#[derive(Clone)]
pub struct Message {
	pub content: Option<String>,
	pub is_from_me: bool,
	pub timestamp: DateTime<Local>,
}

impl Messages {
	pub fn all(&self, phone_number: &str) -> Result<Vec<Message>> {
		let mut handle_sql = self.connection.prepare("SELECT RowID FROM handle WHERE id=?1 AND service=?2")?;
		let mut handle_rows = handle_sql.query(params![phone_number, "iMessage"])?;
		let handle_id: i32 = handle_rows.next()?.unwrap().get(0)?; // TODO: Remove .unwrap()
		
		let mut messages_sql = self.connection.prepare("SELECT text, is_from_me, date FROM message WHERE handle_id=?1")?;
		let mut message_rows = messages_sql.query(params![handle_id])?;

		let mut messages = Vec::new();

		while let Some(row) = message_rows.next()? {
			let raw_timestamp = row.get::<_, i64>(2)? / 1_000_000_000;
			let datetime = NaiveDateTime::from_timestamp(raw_timestamp + TIMESTAMP_OFFSET, 0);

			messages.push(Message {
				content: row.get(0)?,
				is_from_me: row.get::<_, i32>(1)? == 1,
				timestamp: Local.from_utc_datetime(&datetime),
			});
		}

		Ok(messages)
	}
}
