use bincode;
use kafka::producer::{Producer, Record, RequiredAcks};
use schema_registry_converter::async_impl::avro::AvroEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use crate::common::types::SoldCar;

pub async fn create_producer(sold_cars: Arc<RwLock<Vec<SoldCar>>>) {
    let mut producer = Producer::from_hosts(vec![
        "localhost:19092".to_owned(),
        "localhost:29092".to_owned(),
    ])
    .with_ack_timeout(Duration::from_secs(1))
    .with_required_acks(RequiredAcks::One)
    .create()
    .unwrap();

    // Loop over sold_cars and produce them to the Kafka topic
    tokio::spawn(async move {
        let sr_settings = SrSettings::new("http://localhost:8085".to_string());
        let encoder = AvroEncoder::new((sr_settings).clone());
        loop {
            log::debug!("Producing one sold car to Kafka...");
            let mut sold_cars = sold_cars.write().await;
            while let Some(sold_car) = sold_cars.pop() {
                let client_payload = encoder
                    .encode_struct(
                        sold_car,
                        &SubjectNameStrategy::TopicNameStrategy("SoldCar".to_string(), false),
                    )
                    .await
                    .expect("Failed to encode client object");
                producer
                    .send(&Record::from_value("SoldCar", client_payload))
                    .unwrap();
            }
        }
    });
}
