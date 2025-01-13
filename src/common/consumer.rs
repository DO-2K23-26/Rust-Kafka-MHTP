use kafka::{client::GroupOffsetStorage, consumer::{Consumer, FetchOffset}};

pub fn get_consumer(topic: &str) -> Result<Consumer, kafka::error::Error> {
    let consumer = Consumer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_topic_partitions(topic.to_owned(), &[0])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .with_group("group".to_owned())
        .create();
    
    consumer.map_err(|err| err.into())
}