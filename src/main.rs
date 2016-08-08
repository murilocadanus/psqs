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
struct PacketService {
	upstream: SqsClient<ProvideAwsCredentials + 'static std::marker::Sized >,
	//timer: Timer,
}

impl PacketService {
	pub fn new(upstream: SqsClient<ProvideAwsCredentials>) -> PacketService {
		PacketService {
			upstream: upstream,
			//timer: Timer::default(),
		}
	}
}

impl Service for PacketService {
	type Req = String;
	type Resp = String;
	type Error = io::Error;
	type Fut = Box<Future<Item = Self::Resp, Error = io::Error>>;

	fn call(&self, req: Self::Req) -> Self::Fut {
		// Name to be used as queue to post at SQS
		let queue_url = "packages";

		// Remove last char from data sent by equipment
		let msg_str = format!("{}", req);

		// Create a request to be used to post at SQS
		let mut send_req = SendMessageRequest::default();
		send_req.queue_url = queue_url.to_string();
		send_req.message_body = msg_str.to_string();

		// Post message to sqs queue
		let response = try!(self.sqs.send_message(&send_req));

		// Return the response as an immediate future
		match response.message_id {
			Some(message_id) => finished(response.message_body).boxed(),
			None => panic!("Can't post message to queue")
		}
	}
}

fn main() {
	// Init logger API
	env_logger::init().unwrap();

	// Connect to AWS SQS using AWS env vars auth
	let provider = ChainProvider::new();
	let mut sqs = SqsClient::new(provider, Region::SaEast1);

	// Create TCP Server using callback service
	let addr = "127.0.0.1:12345".parse().unwrap();
	pack::Server::new().bind(addr).serve(PacketService::new(sqs)).unwrap();

	thread::sleep(Duration::from_secs(1_000_000));
}

fn sqs_roundtrip_tests <P: ProvideAwsCredentials> (sqs: &mut SqsClient<P>) -> AwsResult<()> {
	debug!("Test logging");

	// list existing queues
	let response = try!(sqs.list_queues(&ListQueuesRequest::default()));
	for q in response.queue_urls {
		println!("Existing queue: {:?}", q);
	}

	// create a new queue
	let q_name = &format!("test_q_{}", get_time().sec);
	let mut req = CreateQueueRequest::default();
	req.queue_name = q_name.to_string();

	let response = try!(sqs.create_queue(&req));
	println!("Created queue {} with url {:?}", q_name, response.queue_url);

	// query it by name
	let mut get_req = GetQueueUrlRequest::default();
	get_req.queue_name = q_name.to_string();
	let response = try!(sqs.get_queue_url(&get_req));
	let queue_url = response.queue_url.unwrap();
	println!("Verified queue url {:?} for queue name {}", queue_url, q_name);

	// send it a message
	let msg_str = "lorem ipsum dolor sit amet";
	let mut send_req = SendMessageRequest::default();
	send_req.queue_url = queue_url.to_string();
	send_req.message_body = msg_str.to_string();

	let response = try!(sqs.send_message(&send_req));
	println!("Send message with body '{}' and created message_id {:?}",
			 msg_str,
			 response.message_id);

	// receive a message
	let mut reci_req = ReceiveMessageRequest::default();
	reci_req.queue_url = queue_url.to_string();
	let response = try!(sqs.receive_message(&reci_req));
	match response.messages {
		Some(messages) => {
			for msg in messages {
				println!("Received message '{:?}' with id {}", msg.body, msg.message_id.unwrap());
				let mut del_msg_req = DeleteMessageRequest::default();
				del_msg_req.queue_url = queue_url.to_string();
				del_msg_req.receipt_handle = msg.receipt_handle.unwrap().to_string();
				try!(sqs.delete_message(&del_msg_req));
			}
		},
		None => println!("no messages")
	}

	// delete the queue
	let mut del_req = DeleteQueueRequest::default();
	del_req.queue_url = queue_url.to_string();
	try!(sqs.delete_queue(&del_req));
	println!("Queue {} deleted", &queue_url);

	Ok(())
}