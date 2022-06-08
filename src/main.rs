mod address_book;
mod manifest;
mod messages;

use clap::Parser;
use manifest::Manifest;
use std::{path::PathBuf, fmt, fs, io};

// UNIX timestamp of Jan 1, 2001 @ 00:00 (Apple's choice)
const TIMESTAMP_OFFSET: i64 = 978307200;
const DATE_FORMAT_STR: &'static str = "%A, %B %d, %Y @ %I:%M %p";

#[derive(Parser)]
struct Args {
	#[clap(parse(from_os_str))]
	backup_path: PathBuf,

	#[clap(parse(from_os_str), short = 'o', long = "output")]
	output_path: PathBuf,
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

	let address_book = manifest.address_book()?;
	let messages = manifest.messages()?;

	fs::create_dir(&args.output_path)?;

	Ok(())
}
