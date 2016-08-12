extern crate rusoto;
extern crate time;

extern crate tokio;
extern crate futures;
extern crate psvr_line as pack;
extern crate rustc_serialize;
extern crate clap;

/*#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_stdlog;*/

#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;


//use slog::drain::IntoLogger;
use clap::{Arg, App};
use std::time::Duration;
use std::thread;

mod service;
mod data;

const NAME: 		&'static str = env!("CARGO_PKG_NAME");
const VERSION: 		&'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: 		&'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: 	&'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
	// Init logger API
	//let log = slog_term::stderr().into_logger(o!("version" => VERSION));
	//slog_stdlog::set_logger(log.clone()).unwrap();

	let format = |record: &LogRecord| {
		let t = time::now();
		format!("[{}.{:03}] - [{}] - {}",
			time::strftime("%d-%m-%Y %H:%M:%S", &t).unwrap(),
			t.tm_nsec / 1000_000,
			record.level(),
			record.args()
		)
	};

	let mut builder = LogBuilder::new();
	builder.format(format).filter(None, LogLevelFilter::Info);

	if env::var("RUST_LOG").is_ok() {
		builder.parse(&env::var("RUST_LOG").unwrap());
	}

	builder.init().unwrap();

	info!("Initialize APP.");

	// Parse parameters at start
	let matches = App::new(NAME)
					.version(VERSION)
					.author(AUTHORS)
					.about(DESCRIPTION)
					.arg(Arg::with_name("port")
						.short("p")
						.long("port")
						.value_name("NUMBER")
						.help("Sets a BIND port to receive data")
						.required(true)
						.takes_value(true))
					.arg(Arg::with_name("equipment")
						.short("e")
						.long("equipment")
						.value_name("EQUIPMENT")
						.help("Sets the equipment to be used")
						.required(true)
						.takes_value(true))
					.get_matches();

	// Gets a value for port if supplied by user
	let port = matches.value_of("port").unwrap();

	// Gets a value for equipment if supplied by user
	let equipment = matches.value_of("equipment").unwrap().to_string();

	// Create TCP Server using callback service
	debug!("Starting TCP server at {}.", port);
	let addr = format!("0.0.0.0:{}", port).parse().unwrap();
	pack::Server::new().bind(addr).serve(service::PacketService::new(equipment)).unwrap();

	// Run forever
	thread::sleep(Duration::from_secs(1_000_000));
}