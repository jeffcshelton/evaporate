use std::path::{PathBuf, Path};
use rusqlite::params;
use clap::Parser;

mod address_book;
mod manifest;

use {
	address_book::AddressBook,
	manifest::Manifest
};

#[derive(Parser)]
struct Args {
	#[clap(parse(from_os_str))]
	backup_path: PathBuf,

	#[clap(short = 'o', long = "output")]
	output_path: Option<PathBuf>,

	#[clap(long = "number")]
	phone_number: String,

	#[clap(long = "recipient")]
	recipient: String,
}

struct Message {
	content: Option<String>,
	is_from_me: bool,
}

fn get_sms_id(manifest_path: &Path) -> rusqlite::Result<String> {
	let manifest_db = rusqlite::Connection::open(manifest_path)?;

	let mut sql = manifest_db.prepare("SELECT fileID FROM Files WHERE relativePath=?1")?;
	let mut rows = sql.query(params!["Library/SMS/sms.db"])?;

	let mut file_ids = Vec::new();
	while let Some(row) = rows.next()? {
		file_ids.push(row.get::<_, String>(0)?.to_string());
	}

	Ok(file_ids[0].clone())
}

fn get_handle_id(messages_db: &rusqlite::Connection, phone_number: &str) -> rusqlite::Result<usize> {
	let mut sql = messages_db.prepare("SELECT rowid FROM handle WHERE id=?1 AND service=?2")?;
	let mut rows = sql.query(params![phone_number, "iMessage"])?;

	let mut handle_ids = Vec::new();
	while let Some(row) = rows.next()? {
		handle_ids.push(row.get::<_, usize>(0)?);
	}

	Ok(handle_ids[0])
}

fn get_messages(messages_db: &rusqlite::Connection, handle_id: usize) -> rusqlite::Result<Vec<Message>> {
	let mut sql = messages_db.prepare("SELECT text, is_from_me FROM message WHERE handle_id=?1")?;
	let mut rows = sql.query(params![handle_id])?;

	let mut messages = Vec::new();
	while let Some(row) = rows.next()? {
		messages.push(Message {
			content: row.get(0)?,
			is_from_me: row.get::<_, i32>(1)? == 1,
		});
	}

	Ok(messages)
}

fn main() -> rusqlite::Result<()> {
	let args = Args::parse();

	let manifest_path = args.backup_path.join("Manifest.db");
	let sms_file_id = get_sms_id(&manifest_path)?;

	println!("SMS File ID: {}", sms_file_id);

	let messages_path = args.backup_path
		.join(&sms_file_id[..2])
		.join(sms_file_id);
	
	let messages_db = rusqlite::Connection::open(messages_path)?;
	let handle_id = get_handle_id(&messages_db, &args.phone_number)?;

	println!("Handle ID: {}", handle_id);

	let messages = get_messages(&messages_db, handle_id)?;
	
	for message in messages {
		println!("[{}]: {}", if message.is_from_me { "me" } else { &args.recipient }, message.content.unwrap_or("<image>".to_owned()));
	}
	
	Ok(())
}
