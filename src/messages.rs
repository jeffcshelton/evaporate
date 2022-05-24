use rusqlite::{Connection as DbConnection, params, Result};

pub struct Messages {
	pub (crate) connection: DbConnection
}

#[derive(Clone)]
pub struct Message {
	pub content: Option<String>,
	pub is_from_me: bool,
}

impl Messages {
	pub fn all(&self, phone_number: &str) -> Result<Vec<Message>> {
		let mut handle_sql = self.connection.prepare("SELECT RowID FROM handle WHERE id=?1 AND service=?2")?;
		let mut handle_rows = handle_sql.query(params![phone_number, "iMessage"])?;
		let handle_id: i32 = handle_rows.next()?.unwrap().get(0)?; // TODO: Remove .unwrap()
		
		let mut messages_sql = self.connection.prepare("SELECT text, is_from_me FROM message WHERE handle_id=?1")?;
		let mut message_rows = messages_sql.query(params![handle_id])?;

		let mut messages = Vec::new();

		while let Some(row) = message_rows.next()? {
			messages.push(Message {
				content: row.get(0)?,
				is_from_me: row.get::<_, i32>(1)? == 1,
			});
		}

		Ok(messages)
	}
}
