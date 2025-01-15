use kafka::{
    client::GroupOffsetStorage,
    consumer::{Consumer, FetchOffset},
};

pub fn get_consumer(topic: &str) -> Result<Consumer, kafka::Error> {
    let consumer = Consumer::from_hosts(vec!["localhost:19092".to_string(),"localhost:29092".to_string()])
        .with_topic_partitions(topic.to_owned(), &[0])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .with_group("order-consumer-group".to_string())
        .create()?;
    
    Ok(consumer)
}
