use bincode;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::common::types::{Chassis, InBuildingCar, Order, Wheel};

pub fn create_consumer(topic: &str, in_building_cars: Arc<RwLock<Vec<InBuildingCar>>>) {
    let mut consumer = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_topic_partitions(topic.to_owned(), &[0, 1])
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
                // Match or if, for each topic of order chassis or wheel, consume their respective topic
                /* "orders" {
                    let order: Order = bincode::deserialize(m.value).unwrap();
                    attach_components(order, in_building_cars);
                }*/
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }
}
