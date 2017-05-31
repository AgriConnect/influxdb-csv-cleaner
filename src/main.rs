//! Tool to clean CSV content exported from InfluxDB.
//!
//! Sample input:
//!
//!    1489544029,29.1,5
//!    1489544039,29.2,5
//!
//! Output with timezone Asia/Ho_Chi_Minh:
//!
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
use chrono::{UTC, TimeZone};
use chrono_tz::Tz;


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

	let mut datetime_string;  // To own a String later
	let mut rows: Vec<String> = Vec::new();

	for wline in reader.lines() {
		let line = wline.unwrap();
		let mut columns: Vec<&str> = line.split(',').collect();
		let timestamp = columns[0].parse::<i64>().unwrap();
		let dest_datetime = UTC.timestamp(timestamp, 0).with_timezone(&dest_timezone);
		datetime_string = dest_datetime.to_string();
		columns[0] = datetime_string.as_str();
		rows.push(columns.join(","));
	}

	let mut writer: Box<Write> = match matches.value_of("output") {
		Some(outfile) => {
			let f = File::create(&Path::new(outfile)).unwrap();
			Box::new(BufWriter::new(f))
		},
		None => Box::new(stdout.lock())
	};
	writeln!(&mut writer, "{}", rows.join("\n")).unwrap();
}
