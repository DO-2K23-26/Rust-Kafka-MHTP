use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

pub fn create_consumer() {
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
