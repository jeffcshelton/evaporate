mod constants;
mod contacts;
mod manifest;
mod messages;

use contacts::Contacts;
use manifest::Manifest;
use messages::Messages;

use clap::Parser;
use std::{path::PathBuf, fmt, fs, io};

#[derive(Parser)]
struct Args {
	#[clap(parse(from_os_str))]
	backup_path: PathBuf,

	#[clap(parse(from_os_str), short = 'o', long = "output")]
	output_path: PathBuf,

	#[clap(long = "no-contacts")]
	no_contacts: bool,

	#[clap(long = "no-messages")]
	no_messages: bool,
}

#[derive(Debug)]
pub enum Error {
	Io(io::ErrorKind),
	Sql(rusqlite::Error),
	NoMessages,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Io(io_error) => write!(f, "{}", io_error.to_string()),
			Self::Sql(sql_error) => write!(f, "{}", sql_error.to_string()),
			Self::NoMessages => write!(f, "no messages"),
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

	let contacts = Contacts::fetch(&manifest)?;
	let messages = Messages::fetch(&manifest, &contacts)?;

	fs::create_dir(&args.output_path)?;

	if !args.no_contacts {
		contacts.extract_to(args.output_path.join("contacts.txt"))?;
	}

	if !args.no_messages {
		messages.extract_to(args.output_path.join("messages"))?;
	}

	Ok(())
}
