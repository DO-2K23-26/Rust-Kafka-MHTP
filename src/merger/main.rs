use Rust_Kafka_MHT::common::types::{InBuildingCar, SoldCar};
use Rust_Kafka_MHT::merger::consumer::create_consumer;
use Rust_Kafka_MHT::merger::producer::create_producer;

async fn main() {
    // Initialize the in-memory storage of cars being built and cars being sold
    let in_building_cars: Arc<RwLock<Vec<InBuildingCar>>> = Arc::new(RwLock::new(Vec::new()));
    let sold_cars: Arc<RwLock<Vec<SoldCar>>> = Arc::new(RwLock::new(Vec::new()));

    // Initialize the JoinSet for all threads
    let client_set = JoinSet::new();
    let client_arc = Arc::new(RwLock::new(client_set));

    create_producer(client_arc.clone(), sold_cars.clone());
    create_consumer::<Order>(client_arc.clone(), in_building_cars.clone(), sold_cars.clone());
    create_consumer::<Chassis>(client_arc.clone(), in_building_cars.clone(), sold_cars.clone());
    create_consumer::<Wheel>(client_arc.clone(), in_building_cars.clone(), sold_cars.clone());
    // Await all tasks
    let mut producer_lock = producer_arc.write().unwrap();
    while let Some(result) = producer_lock.join_next().await {
        if let Err(e) = result {
            eprintln!("Client task failed: {:?}", e);
        }
    }
}
