use rusqlite::{Connection as DbConnection, params};

use chrono::{
	Date,
	Local,
	NaiveDateTime,
	TimeZone,
};

use crate::{
	constants::{
		DATE_FORMAT_STR,
		TIMESTAMP_OFFSET,
	},
	manifest::Manifest,
	Result,
};

use std::{
	fs::File,
	io::Write,
	path::Path,
};

pub struct Contact {
	pub first_name: String,
	pub middle_name: Option<String>,
	pub last_name: Option<String>,
	pub nickname: Option<String>,
	pub prefix: Option<String>,
	pub suffix: Option<String>,

	pub phone_number: Option<String>,
	pub email: Option<String>,

	pub organization: Option<String>,
	pub department: Option<String>,
	pub job_title: Option<String>,

	pub birthday: Option<Date<Local>>,
	pub anniversary: Option<Date<Local>>,
	pub note: Option<String>,
}

impl ToString for Contact {
	fn to_string(&self) -> String {
		let mut ret = "[".to_owned();
		
		if let Some(prefix) = &self.prefix {
			ret.push_str(&prefix);
			ret.push(' ');
		}

		ret += &self.first_name;

		if let Some(middle_name) = &self.middle_name {
			ret.push(' ');
			ret.push_str(&middle_name);
		}

		if let Some(last_name) = &self.last_name {
			ret.push(' ');
			ret.push_str(&last_name);
		}

		if let Some(nickname) = &self.nickname {
			ret.push_str(" (");
			ret.push_str(&nickname);
			ret.push(')');
		}

		ret.push_str("]:");

		if let Some(phone_number) = &self.phone_number {
			ret.push_str("\nPhone Number: ");
			ret.push_str(&phone_number);
		}

		if let Some(email) = &self.email {
			ret.push_str("\nEmail: ");
			ret.push_str(&email);
		}

		if let Some(organization) = &self.organization {
			ret.push_str("\nOrganization: ");
			ret.push_str(&organization);
		}

		if let Some(department) = &self.department {
			ret.push_str("\nDepartment: ");
			ret.push_str(&department);
		}

		if let Some(job_title) = &self.job_title {
			ret.push_str("\nJob Title: ");
			ret.push_str(&job_title);
		}

		if let Some(birthday) = self.birthday {
			ret.push_str("\nBirthday: ");
			ret.push_str(&birthday.format(DATE_FORMAT_STR).to_string());
		}

		if let Some(anniversary) = self.anniversary {
			ret.push_str("\nAnniversary: ");
			ret.push_str(&anniversary.format(DATE_FORMAT_STR).to_string());
		}

		if let Some(note) = &self.note {
			ret.push_str("\nNote: ");
			ret.push_str(&note);
		}

		ret.push('\n');

		ret
	}
}

fn fetch(manifest: &Manifest) -> Result<Vec<Contact>> {
	let connection = DbConnection::open(manifest.get_path("Library/AddressBook/AddressBook.sqlitedb")?)?;

	let mut sql = connection.prepare("
		SELECT
			Person.First,
			Person.Middle,
			Person.Last,
			Person.Nickname,
			Person.Prefix,
			Person.Suffix,
			PhoneNumber.value,
			Email.value,
			Person.Organization,
			Person.Department,
			Person.JobTitle,
			CAST(Person.Birthday AS INT),
			CAST(Anniversary.value AS INT),
			Person.Note
		FROM ABPerson AS Person
		LEFT JOIN ABMultiValue AS Anniversary
			ON Person.RowID=Anniversary.record_id AND Anniversary.property=?1
		LEFT JOIN ABMultiValue AS PhoneNumber
			ON Person.RowID=PhoneNumber.record_id AND PhoneNumber.property=?2
		LEFT JOIN ABMultiValue AS Email
			ON Person.RowID=Email.record_id AND Email.property=?3
		WHERE Person.First IS NOT NULL
	")?;

	let mut rows = sql.query(params![12_i32, 3_i32, 4_i32])?;
	let mut contacts = Vec::new();

	while let Some(row) = rows.next()? {
		let phone_number = row.get::<_, Option<String>>(6)?
			.map(|mut num| {
				num = num.replace(['(', ')', ' ', '-'], "");

				if !num.starts_with('+') {
					num = "+1".to_owned() + &num;
				}

				num
			});

		let birthday = row.get::<_, Option<i64>>(11)?
			.map(|timestamp| {
				Local.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp + TIMESTAMP_OFFSET, 0)).date()
			});

		let anniversary = row.get::<_, Option<i64>>(12)?
			.map(|timestamp| {
				Local.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp + TIMESTAMP_OFFSET, 0)).date()
			});

		contacts.push(Contact {
			first_name: row.get(0)?,
			middle_name: row.get(1)?,
			last_name: row.get(2)?,
			nickname: row.get(3)?,
			prefix: row.get(4)?,
			suffix: row.get(5)?,
			phone_number: phone_number,
			email: row.get(7)?,
			organization: row.get(8)?,
			department: row.get(9)?,
			job_title: row.get(10)?,
			birthday: birthday,
			anniversary: anniversary,
			note: row.get(13)?,
		});
	}

	Ok(contacts)
}

pub fn extract_to<P: AsRef<Path>>(path: P, manifest: &Manifest) -> Result<()> {
	let path = path.as_ref();
	let mut file = File::create(path)?;

	for contact in fetch(manifest)? {
		file.write_all((contact.to_string() + "\n").as_bytes())?;
	}

	Ok(())
}
