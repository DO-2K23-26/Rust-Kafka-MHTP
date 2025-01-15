use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};
use bincode;

use crate::common::types::{SoldCar, Brand};

pub fn create_producer() {
  let mut producer = Producer::from_hosts(vec!("localhost:9092".to_owned()))
    .with_ack_timeout(Duration::from_secs(1))
    .with_required_acks(RequiredAcks::One)
    .create()
    .unwrap();

  for _ in 0..10 {
    // Here call the merge logic function
    let pwetter = SoldCar {
      id: 1,
      brand: Brand::FERRARI,
      price: 100.0190382110349729845762347,
      created_at: 1234567890,
    };
    let encoded: Vec<u8> = bincode::serialize(&pwetter).unwrap();
    producer.send(&Record::from_value("orders", encoded.as_slice())).unwrap();
  }
}
