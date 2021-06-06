use amiquip::{Connection, QueueDeclareOptions, ConsumerMessage, ConsumerOptions, Result};



fn main() -> Result<()> {
    let mut conn = Connection::insecure_open("amqp://rabbit:rabbit@localhost:5672").unwrap();
    let channel = conn.open_channel(None).unwrap();

    let queue = channel.queue_declare("hello", QueueDeclareOptions::default()).unwrap();

    let consumer = queue.consume(ConsumerOptions::default()).unwrap();
    println!("Waiting for messages. Press Ctrl-C to exit.");
    
    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);
                consumer.ack(delivery).unwrap();
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    conn.close()

}
