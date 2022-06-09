mod constants;
mod contacts;
mod manifest;
mod messages;
mod photos;

use clap::Parser;
use manifest::Manifest;
use std::{path::PathBuf, fmt, fs, io, process};

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

	#[clap(long = "no-photos")]
	no_photos: bool,
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

fn main() {
	let args = Args::parse();
	let manifest = Manifest::open(&args.backup_path)
		.unwrap_or_else(|_| {
			panic!();
		});

	fs::create_dir(&args.output_path).unwrap_or_else(|error| {
		println!("\x1b[31m! Could not create output directory: {} !\x1b[0m", error);
		process::exit(1);
	});

	if !args.no_contacts {
		match contacts::extract_to(args.output_path.join("contacts.txt"), &manifest) {
			Ok(()) => println!("\x1b[32mSuccessfully extracted contacts.\x1b[0m"),
			Err(error) => println!("\x1b[33m! Failed to extract contacts: {} !\x1b[0m", error),
		};
	}

	if !args.no_messages {
		match messages::extract_to(args.output_path.join("messages"), &manifest) {
			Ok(()) => println!("\x1b[32mSuccessfully extracted messages.\x1b[0m"),
			Err(error) => println!("\x1b[33m! Failed to extract messages: {} !\x1b[0m", error),
		};
	}

	if !args.no_photos {
		match photos::extract_to(args.output_path.join("photos"), &manifest) {
			Ok(()) => println!("\x1b[32mSuccessfully extracted photos.\x1b[0m"),
			Err(error) => println!("\x1b[33m! Failed to extract photos: {} !\x1b[0m", error),
		};
	}
}
