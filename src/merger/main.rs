use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::task::JoinSet;
use Rust_Kafka_MHT::common::types::{Chassis, Order, SoldCar, Wheel};
use Rust_Kafka_MHT::merger::consumer::create_consumer;
use Rust_Kafka_MHT::merger::logic::InBuildingCar;
use Rust_Kafka_MHT::merger::producer::create_producer;

#[tokio::main]
async fn main() {
    // Initialize the in-memory storage of cars being built and cars being sold
    let in_building_cars: Arc<RwLock<Vec<InBuildingCar>>> = Arc::new(RwLock::new(Vec::new()));
    let sold_cars: Arc<RwLock<Vec<SoldCar>>> = Arc::new(RwLock::new(Vec::new()));

    // Initialize the JoinSet for all threads
    let client_set = JoinSet::new();
    let client_arc = Arc::new(RwLock::new(client_set));

    let _ = create_producer(client_arc.clone(), sold_cars.clone()).await;
    let _ = create_consumer::<Order>(
        client_arc.clone(),
        in_building_cars.clone(),
        sold_cars.clone(),
    ).await;
    let _ = create_consumer::<Chassis>(
        client_arc.clone(),
        in_building_cars.clone(),
        sold_cars.clone(),
    ).await;
    let _ = create_consumer::<Wheel>(
        client_arc.clone(),
        in_building_cars.clone(),
        sold_cars.clone(),
    ).await;
    // Await all tasks
    let mut client_lock = client_arc.write().await;
    while let Some(result) = client_lock.join_next().await {
        if let Err(e) = result {
            eprintln!("Client task failed: {:?}", e);
        }
    }
}
