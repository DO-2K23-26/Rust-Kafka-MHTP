use Rust_Kafka_MHT::common::types::InBuildingCar;
use Rust_Kafka_MHT::merger::consumer::create_consumer;
use Rust_Kafka_MHT::merger::producer::create_producer;

fn main() {
    // Initialize the in-memory storage of cars being built
    let in_building_cars: Arc<RwLock<Vec<InBuildingCar>>> = Arc::new(RwLock::new(Vec::new()));

    create_producer(in_building_cars.clone());
    create_consumer("orders", in_building_cars.clone());
    create_consumer("chassis", in_building_cars.clone());
    create_consumer("wheels", in_building_cars.clone());
}
