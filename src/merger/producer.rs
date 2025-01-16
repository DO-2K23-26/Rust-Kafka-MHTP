use bincode;
use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinSet;

use crate::common::types::SoldCar;

pub async fn create_producer(client_set: Arc<RwLock<JoinSet<()>>>, sold_cars: Arc<RwLock<Vec<SoldCar>>>) {
    let producer = Producer::from_hosts(vec!["localhost:19092".to_owned(),"localhost:29092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let client_arc = Arc::new(RwLock::new(producer));
    let client_clone = client_arc.clone();
    let mut client_lock = client_set.write().await;

    // Loop over sold_cars and produce them to the Kafka topic
    client_lock.spawn(async move {
        let mut producer_lock = client_clone.write().await;
        loop {
            let sold_cars = sold_cars.read().await;
            for sold_car in sold_cars.iter() {
                let encoded: Vec<u8> = bincode::serialize(&sold_car).unwrap();
                producer_lock
                    .send(&Record::from_value("sold_cars", encoded.as_slice()))
                    .unwrap();
            }
        }
    });
}
