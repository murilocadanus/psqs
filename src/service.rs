use std::io;
use tokio::Service;
use futures::{Future, finished};
use rusoto::{ChainProvider, Region};
use rusoto::sqs::{SqsClient, GetQueueUrlRequest, SendMessageRequest, CreateQueueRequest};
use ::data::EquipmentType;

#[derive(Clone)]
pub struct PacketService {
	equipment_type: String,
	queue_url: String,
}

impl PacketService {
	pub fn new(equipment_type: String) -> PacketService {
		// Use or create queue based on equipment name
		let queue_url = match equipment_type.parse::<EquipmentType>() {
			Ok(_type) => {
				// Create AWS SQS client using AWS env vars auth
				let provider = ChainProvider::new();
				let sqs = SqsClient::new(provider, Region::SaEast1);

				// Create a queue url request by name
				let mut get_req = GetQueueUrlRequest::default();
				get_req.queue_name = equipment_type.to_string();

				// Set queue url and message
				let msg_url = match sqs.get_queue_url(&get_req) {
					Ok(resp) => format!("{}", resp.queue_url.unwrap()),
					Err(error) => {
						// Create a new queue with random name
						let mut req = CreateQueueRequest::default();
						req.queue_name = equipment_type.to_string();

						// Call and verify if has a valid result
						match sqs.create_queue(&req) {
							Ok(resp) => resp.queue_url.unwrap().to_string(),
							Err(error) => panic!("AWS SQS Error: Can't create queue, {:?}", error),
						}
					},
				};
			},
			Err(()) => panic!("Invalid equipment type!"),
		};

		PacketService { equipment_type: equipment_type, queue_url: queue_url}
	}
}

impl Service for PacketService {
	type Req = String;
	type Resp = String;
	type Error = io::Error;
	type Fut = Box<Future<Item = Self::Resp, Error = io::Error>>;

	fn call(&self, req: Self::Req) -> Self::Fut {
		// Create AWS SQS client using AWS env vars auth
		let provider = ChainProvider::new();
		let sqs = SqsClient::new(provider, Region::SaEast1);

		// Create send message request
		let mut send_req = SendMessageRequest::default();
		send_req.queue_url = self.queue_url.to_string();
		send_req.message_body = req.to_string();

		// Send message
		match sqs.send_message(&send_req) {
			Ok(message) => println!("AWS SQS Success: Sent message {:?}", message),
			Err(error) => panic!("AWS SQS Error: Can't post at queue, {:?}", error),
		}

		// Return acknowledgement
		finished("ACK".to_string()).boxed()
	}
}