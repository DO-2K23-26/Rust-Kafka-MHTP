use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use kafka::{
    client::{KafkaClient, RequiredAcks},
    producer::{Producer, Record},
};
use tokio::task::JoinSet;
use Rust_Kafka_MHT::common::types::{Chassis, Emittable, Order, Wheel};

pub fn launch_producer<T>(producer_set: Arc<RwLock<JoinSet<()>>>)
where
    T: Emittable + Send + Sync + 'static,
{
    let mut client = KafkaClient::new(vec!["localhost:9092".to_owned()]);
    client.load_metadata_all().unwrap();

    let producer = Producer::from_client(client)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();
    let producer_arc = Arc::new(RwLock::new(producer));
    let producer_clone = producer_arc.clone();
    let mut producer_lock = producer_set.write().unwrap();

    producer_lock.spawn(async move {
        let mut producer_lock = producer_clone.write().unwrap();
        loop {
            let topic_name = T::get_topic_name();
            let _ = producer_lock.send(&Record::from_value(
                &topic_name,
                serde_json::to_string(&T::generate()).unwrap(),
            ));
            std::thread::sleep(Duration::from_millis(50));
        }
    });
}

#[tokio::main]
async fn main() {
    let producer_set = JoinSet::new();
    let producer_arc = Arc::new(RwLock::new(producer_set));
    // Clone the client for each producer
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
