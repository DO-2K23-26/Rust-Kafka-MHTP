use bincode;
use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::common::types::SoldCar;

pub async fn create_producer(sold_cars: Arc<RwLock<Vec<SoldCar>>>) {
    let mut producer = Producer::from_hosts(vec!["localhost:19092".to_owned(),"localhost:29092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    // Loop over sold_cars and produce them to the Kafka topic
    tokio::spawn(async move {
        loop {
            log::debug!("Producing one sold car to Kafka...");
            let sold_cars = sold_cars.read().await;
            for sold_car in sold_cars.iter() {
                let encoded: Vec<u8> = bincode::serialize(&sold_car).unwrap();
                producer
                    .send(&Record::from_value("SoldCar", encoded.as_slice()))
                    .unwrap();
            }
        }
    });
}
