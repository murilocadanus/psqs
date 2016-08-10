extern crate rusoto;
extern crate time;

extern crate tokio;
extern crate futures;
extern crate psvr_line as pack;
extern crate rustc_serialize;

#[macro_use]
extern crate log;
extern crate env_logger;

use rusoto::{AwsResult, ChainProvider, Region, ProvideAwsCredentials};
use rusoto::sqs::{SqsClient, CreateQueueRequest, ListQueuesRequest, GetQueueUrlRequest,
	SendMessageRequest, ReceiveMessageRequest, DeleteMessageRequest, DeleteQueueRequest};
use time::get_time;
use std::time::Duration;

use tokio::Service;
use futures::{Future, finished};
use std::{thread, io};

mod data;

#[derive(Clone)]
struct PacketService;

impl Service for PacketService {
	type Req = String;
	type Resp = String;
	type Error = io::Error;
	type Fut = Box<Future<Item = Self::Resp, Error = io::Error>>;

	fn call(&self, req: Self::Req) -> Self::Fut {

		println!("call");

		// Connect to AWS SQS using AWS env vars auth
		let provider = ChainProvider::new();
		let mut sqs = SqsClient::new(provider, Region::SaEast1);

		// Name to be used as queue to post at SQS
		let queue_url = "packages";

		// Remove last char from data sent by equipment
		let msg_str = format!("{}", req);

		// Create a request to be used to post at SQS
		let mut send_req = SendMessageRequest::default();
		send_req.queue_url = queue_url.to_string();
		send_req.message_body = msg_str.to_string();

		// Post message to sqs queue
		let response = try!(sqs.send_message(&send_req));

		// Return the response as an immediate future
		match response.message_id {
			Some(message_id) => finished(message_id).boxed(),
			None => panic!("Can't post message to queue")
		}
		finished("Ack".to_string()).boxed()
	}
}

fn main() {
	// Init logger API
	env_logger::init().unwrap();

	// Create TCP Server using callback service
	let addr = "127.0.0.1:12345".parse().unwrap();
	pack::Server::new().bind(addr).serve(PacketService/*::new(sqs)*/).unwrap();

	thread::sleep(Duration::from_secs(1_000_000));
}