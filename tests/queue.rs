extern crate rusoto;
extern crate time;

#[cfg(test)]
mod tests {

	use rusoto::{ChainProvider, Region};
	use rusoto::sqs::{
		SqsClient, CreateQueueRequest, ListQueuesRequest, GetQueueUrlRequest,
		SendMessageRequest, ReceiveMessageRequest, DeleteMessageRequest, DeleteQueueRequest
	};
	use time::get_time;
	use std::time::Duration;

	#[test]
	fn list_queues() {
		// Connect to AWS SQS using AWS env vars auth
		let provider = ChainProvider::new();
		let sqs = SqsClient::new(provider, Region::SaEast1);

		// Call and verify if has a valid result
		let response = sqs.list_queues(&ListQueuesRequest::default());
		assert!(response.is_ok());
	}

	#[test]
	fn create_queue() {
		// Connect to AWS SQS using AWS env vars auth
		let provider = ChainProvider::new();
		let sqs = SqsClient::new(provider, Region::SaEast1);

		// Create a new queue with random name
		let q_name = &format!("test_q_{}", get_time().sec);
		let mut req = CreateQueueRequest::default();
		req.queue_name = q_name.to_string();

		// Call and verify if has a valid result
		let response = sqs.create_queue(&req);
		assert!(response.is_ok());
	}

	#[test]
	fn get_queue_by_name() {
		// Connect to AWS SQS using AWS env vars auth
		let provider = ChainProvider::new();
		let sqs = SqsClient::new(provider, Region::SaEast1);

		// Query it by name
		let mut get_req = GetQueueUrlRequest::default();
		get_req.queue_name = "packages".to_string();

		// Call and verify if has a valid result
		let response = sqs.get_queue_url(&get_req);
		assert!(response.is_ok());
	}

	#[test]
	fn set_message_queue() {
		// Connect to AWS SQS using AWS env vars auth
		let provider = ChainProvider::new();
		let sqs = SqsClient::new(provider, Region::SaEast1);

		// Create a queue url request by name
		let mut get_req = GetQueueUrlRequest::default();
		get_req.queue_name = "packages".to_string();

		// Set queue url and message
		let msg_url = sqs.get_queue_url(&get_req).ok().unwrap().queue_url.unwrap();
		let msg_str = "lorem ipsum dolor sit amet";

		// Create send message request
		let mut send_req = SendMessageRequest::default();
		send_req.queue_url = msg_url.to_string();
		send_req.message_body = msg_str.to_string();

		// Call and verify if a message was sent
		let response = sqs.send_message(&send_req);
		assert!(response.is_ok());
	}

/*
	#[test]
	fn consume_messages_queue() {
		// Connect to AWS SQS using AWS env vars auth
		let provider = ChainProvider::new();
		let mut sqs = SqsClient::new(provider, Region::SaEast1);

		// Create a queue url request by name
		let mut get_req = GetQueueUrlRequest::default();
		get_req.queue_name = "packages".to_string();
		let queue_url = sqs.get_queue_url(&get_req).ok().unwrap().queue_url.unwrap();

		// Create receive message request
		let mut reci_req = ReceiveMessageRequest::default();
		reci_req.queue_url = queue_url;

		let response = sqs.receive_message(&reci_req);
		match response.ok().unwrap().messages {
			Some(messages) => {
				// Contain all return status of delete
				//let mut results = vec![];

				// Iterate messages to delete
				for msg in messages {
					// Create a delete message request
					let mut del_msg_req = DeleteMessageRequest::default();
					del_msg_req.queue_url = queue_url;
					del_msg_req.receipt_handle = msg.receipt_handle.unwrap().to_string();

					let result = sqs.delete_message(&del_msg_req);
					//results.push(result.is_ok());

					assert!(result.is_ok());
				}

			},
			None => assert!(true),
		}
	}

	#[test]
	fn delete_queue() {
		// Connect to AWS SQS using AWS env vars auth
		let provider = ChainProvider::new();
		let mut sqs = SqsClient::new(provider, Region::SaEast1);

		// query it by name
		let mut get_req = GetQueueUrlRequest::default();
		let q_name = "packages";
		get_req.queue_name = q_name.to_string();
		let response = sqs.get_queue_url(&get_req);
		let queue_url = response.queue_url.unwrap();

		// delete the queue
		let mut del_req = DeleteQueueRequest::default();
		del_req.queue_url = queue_url.to_string();
		sqs.delete_queue(&del_req);
		println!("Queue {} deleted", &queue_url);
	}
*/
}