mod address_book;
mod manifest;
mod messages;

use clap::{Parser, Subcommand};
use manifest::Manifest;
use std::{path::PathBuf, fmt, io};

#[derive(Subcommand)]
enum Action {
	All,
	Messages {
		#[clap(long = "contact")]
		contact_name: Option<String>
	},
}

#[derive(Parser)]
struct Args {
	#[clap(subcommand)]
	action: Action,

	#[clap(parse(from_os_str))]
	backup_path: PathBuf,

	#[clap(parse(from_os_str), short = 'o', long = "output")]
	output_path: PathBuf,
}

#[derive(Debug)]
pub enum Error {
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

	match args.action {
		Action::All => {
			manifest.messages()?.extract_all(&args.output_path)?;
		},
		Action::Messages { contact_name } => {
			if let Some(contact_name) = contact_name {
				let contact = manifest.address_book()?.get_contact(&contact_name)?;
				manifest.messages()?.extract(contact, &args.output_path)?;
			} else {
				unimplemented!();
			}
		}
	}
	
	Ok(())
}
