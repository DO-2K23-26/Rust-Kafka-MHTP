use bincode;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

use crate::common::types::{Chassis, Order, Wheel};

pub fn create_consumer() {
    let mut messages = Vec::new(); // List to store the retrieved messages.
    let mut consumer = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_topic_partitions("orders".to_owned(), &[0, 1])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("merger".to_owned())
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .unwrap();
    loop {
        // Retrieve the MessageSet. There is at most one per topic.
        for ms in consumer.poll().unwrap().iter() {
            // Loop on each message of the MessageSet.
            for m in ms.messages() {
                /*
                client code accesses the raw data/bytes, parses it into custom data types,
                and passes that along for further processing within the application.
                */
                messages.push(m.value.to_vec()); // Persist temporary message references into our "messages" vector.
                println!("{:?}", messages);
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }
}
