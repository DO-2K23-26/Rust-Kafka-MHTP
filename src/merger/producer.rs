use std::fmt::Write;
use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};
use serde::{Serialize, Deserialize};
use bincode;

pub fn create_producer() {
  let mut producer = Producer::from_hosts(vec!("localhost:9092".to_owned()))
    .with_ack_timeout(Duration::from_secs(1))
    .with_required_acks(RequiredAcks::One)
    .create()
    .unwrap();

  /*let mut buf = Vec::new();
  write!(&mut buf, "message").unwrap();
  producer.send(&Record {
    topic: "my-topic",
    partition: 0,
    key: "key",
    value: &buf,
  }).unwrap();*/

  let mut buf = String::with_capacity(2);
  for i in 0..10 {
    let _ = write!(&mut buf, "{}", i); // some computation of the message data to be sent
    println!("Sending message: {}", buf);
    producer.send(&Record::from_value("orders", buf.as_bytes())).unwrap();
    buf.clear();
  }

  #[derive(Serialize, Deserialize)]
  struct LePwet {
    pwetting_level: String,
  }
  let pwetter = LePwet { pwetting_level: String::from("aae") };

  let encoded: Vec<u8> = bincode::serialize(&pwetter).unwrap();
  producer.send(&Record::from_value("orders", encoded.as_slice())).unwrap();
}
