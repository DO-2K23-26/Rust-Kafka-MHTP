use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use kafka::{
    client::RequiredAcks,
    producer::{Producer, Record},
};
use schema_register::register;
use schema_registry_converter::async_impl::avro::AvroEncoder;
use schema_registry_converter::{
    async_impl::schema_registry::SrSettings, schema_registry_common::SubjectNameStrategy,
};

use serde_avro_derive::BuildSchema;
use tokio::task::JoinSet;
use Rust_Kafka_MHT::common::types::{Chassis, Emittable, Order, SoldCar, Wheel};

mod schema_register;

pub fn launch_producer<T>(producer_set: Arc<RwLock<JoinSet<()>>>)
where
    T: Emittable + Send + Sync + BuildSchema + 'static,
{
    let mut producer = Producer::from_hosts(vec![
        "localhost:19092".to_owned(),
        "localhost:29092".to_owned(),
    ])
    .with_ack_timeout(Duration::from_secs(1))
    .with_required_acks(RequiredAcks::One)
    .create()
    .unwrap();
    let mut producer_lock = producer_set.write().unwrap();
    producer_lock.spawn(async move {
        let sr_settings = SrSettings::new(String::from("http://localhost:8085"));
        let encoder = AvroEncoder::new((sr_settings).clone());
        loop {
            let topic_name = T::get_topic_name();
            let client_payload = encoder
                .encode_struct(
                    &T::generate(),
                    &SubjectNameStrategy::TopicNameStrategy(T::get_topic_name(), false),
                )
                .await
                .expect("Failed to encode client object");

            let _ = producer.send(&Record::from_value(&topic_name, client_payload));
            std::thread::sleep(Duration::from_millis(T::get_frequency()));
        }
    });
}

#[tokio::main]
async fn main() {
    register::<Order>().await;
    register::<Wheel>().await;
    register::<Chassis>().await;
    register::<SoldCar>().await;

    let producer_set = JoinSet::new();
    let producer_arc = Arc::new(RwLock::new(producer_set));

    launch_producer::<Order>(producer_arc.clone());
    launch_producer::<Wheel>(producer_arc.clone());
    launch_producer::<Chassis>(producer_arc.clone());
    // Await all tasks
    let mut producer_lock = producer_arc.write().unwrap();
    while let Some(result) = producer_lock.join_next().await {
        if let Err(e) = result {
            eprintln!("Producer task failed: {:?}", e);
        }
    }
}
