//! Tool to clean CSV content exported from InfluxDB.
//!
//! Sample input:
//!
//!    name,time,temperature,crop_id
//!    condition,1489544029,29.1,5
//!    condition,1489544039,29.2,5
//!
//! Output with timezone Asia/Ho_Chi_Minh:
//!
//!    time,temperature,crop_id
//!    2017-03-15 09:13:49 +07,29.1,5
//!    2017-03-15 09:13:59 +07,29.2,5
//!

#[macro_use]
extern crate clap;
extern crate chrono;
extern crate chrono_tz;

use std::io;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use clap::{Arg, App};
use chrono::{TimeZone};
use chrono_tz::Tz;


fn concat_columns(last: String, current: &str) -> String {
	last + "," + current
}


fn process_line(line: String, on_first_line: bool, dest_timezone: Tz) -> Option<String> {
	let mut column_iter = line.split(',');
	// First column is measurement name. Skip it
	column_iter.next();
	// The second column becomes the first, and should contain timestamp in UTC
	let date_column = column_iter.next().unwrap();
	let timestamp = date_column.parse::<i64>();
	if timestamp.is_err() {
		// If the line cannot be parsed, and is the first line of file,
		// it is the header of CSV and we just return original to output
		if on_first_line {
			let new_line = column_iter.fold(date_column.to_string(), concat_columns);
			return Some(new_line);
		}
		// If not the first line, print error and continue to next line
		writeln!(&mut io::stderr(), "Line {} doesn't starts with timestamp", line).unwrap();
		return None;
	}
	let dest_datetime = dest_timezone.timestamp(timestamp.unwrap(), 0).to_string();
	let new_line = column_iter.fold(dest_datetime, concat_columns);
	Some(new_line)
}


fn main() {
	let matches = App::new("Cleaning InfluxDB's export CSV")
		.version(crate_version!()).author(crate_authors!())
		.about("Clean CSV file, exported from InfluxDB.\n\
		       Remove the first column (measurement name) and convert timezone for time column")
		.arg(Arg::with_name("INPUT")
		     .help("Input file name. - for stdin.")
		     .required(true))
		.arg(Arg::with_name("timezone")
		     .short("t")
		     .value_name("Timezone name")
		     .help("Timezone (Asia/Ho_Chi_Minh) to convert to. \n
		           Original InfluxDB CSV has time in UTC."))
		.arg(Arg::with_name("output")
		     .short("o")
		     .value_name("file name")
		     .help("Output file name. Ommit to write to stdout."))
		.get_matches();

	let infile = matches.value_of("INPUT").unwrap();
	let timezone_name = matches.value_of("timezone");
	let mut dest_timezone: Tz = chrono_tz::UTC;
	let stdout = io::stdout();
	if timezone_name.is_some() {
		let parsed = timezone_name.unwrap().parse();
		dest_timezone = parsed.expect("Invalid timezone!");
	}
	let stdin = io::stdin();
	let mut reader = Box::new(stdin.lock()) as Box<BufRead>;

	if infile != "-" {
		let f = File::open(infile).expect("File not found.");
		reader = Box::new(BufReader::new(f));
	}

	let mut rows: Vec<String> = Vec::new();

	for (i, wline) in reader.lines().enumerate() {
		let line = wline.unwrap();
		process_line(line, i == 0, dest_timezone).map(|l| rows.push(l));
	}

	let mut writer: Box<Write> = match matches.value_of("output") {
		Some(outfile) => {
			let created = File::create(&Path::new(outfile));
			assert!(created.is_ok(), "Failed. Cannot create file: {}", outfile);
			Box::new(BufWriter::new(created.unwrap()))
		},
		None => Box::new(stdout.lock())
	};
	writeln!(&mut writer, "{}", rows.join("\n")).unwrap();
}
