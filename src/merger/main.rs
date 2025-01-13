use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

fn create_consumer() {
  let mut consumer =
     Consumer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_topic_partitions("orders".to_owned(), &[0, 1])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("merger".to_owned())
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .unwrap();
  loop {
    for ms in consumer.poll().unwrap().iter() {
      for m in ms.messages() {
        println!("{:?}", m);
      }
      consumer.consume_messageset(ms);
    }
    consumer.commit_consumed().unwrap();
  }
}
/*
use std::fmt::Write;
use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};

fn create_producer() {
  let mut producer = Producer::from_hosts(vec!("localhost:9092".to_owned()))
    .with_ack_timeout(Duration::from_secs(1))
    .with_required_acks(RequiredAcks::One)
    .create()
    .unwrap();

  let mut buf = Vec::new();
  write!(&mut buf, "message").unwrap();
  producer.send(&Record {
    topic: "my-topic",
    partition: 0,
    key: "key",
    value: &buf,
  }).unwrap();

  let mut buf = String::with_capacity(2);
  for i in 0..10 {
    let _ = write!(&mut buf, "{}", i); // some computation of the message data to be sent
    producer.send(&Record::from_value("my-topic", buf.as_bytes())).unwrap();
    buf.clear();
  }
}*/

fn main() {
  create_consumer();
  //create_producer();
}
