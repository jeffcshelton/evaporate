use std::io::Write;

use chrono::Duration;

mod address_book;
mod manifest;
mod messages;

use {
	clap::Parser,
	manifest::Manifest,
	std::{
		path::PathBuf,
		fmt,
		fs::File,
		io
	},
};

#[derive(Parser)]
struct Args {
	#[clap(parse(from_os_str))]
	backup_path: PathBuf,

	#[clap(short = 'o', long = "output")]
	output_path: Option<PathBuf>,

	#[clap(long)]
	name: String,
}

#[derive(Debug)]
enum Error {
	Io(io::ErrorKind),
	Sql(rusqlite::Error),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Io(io_error) => write!(f, "{}", io_error.to_string()),
			Self::Sql(sql_error) => write!(f, "{}", sql_error.to_string()),
		}
	}
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::Io(error.kind())
	}
}

impl From<rusqlite::Error> for Error {
	fn from(error: rusqlite::Error) -> Self {
		Error::Sql(error)
	}
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
	let args = Args::parse();

	let manifest = Manifest::open(&args.backup_path)?;
	let address_book = manifest.address_book()?;
	let messages_db = manifest.messages()?;

	let phone_number = address_book.get_phone_number(&args.name)?;
	let messages = messages_db.all(&phone_number)?;

	if let Some(output_path) = args.output_path {
		let mut file = File::create(output_path)?;

		for m in 0..messages.len() {
			let msg = &messages[m];
			let last_msg = messages.get(m - 1);

			if (last_msg.is_some() && msg.timestamp - last_msg.unwrap().timestamp > Duration::hours(2)) || (last_msg.is_none()) {
				file.write_all(
					format!("\n      - {} -\n\n", msg.timestamp.format("%A, %B %d, %Y at %I:%M %p"))
						.as_bytes()
				)?;
			}

			file.write_all(
				format!("[{}]: {}\n",
					if msg.is_from_me { "me" } else { &args.name },
					msg.content.clone().unwrap_or("<image>".to_owned()),
				).as_bytes(),
			)?;
		}
	} else {
		for message in messages {
			println!("[{}]: {}", if message.is_from_me { "me" } else { &args.name }, message.content.unwrap_or("<image>".to_owned()));
		}
	}
	
	Ok(())
}
