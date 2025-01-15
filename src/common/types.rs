use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Brand {
    FERRARI,
    RENAULT,
    PEUGEOT,
    CITROEN,
    BMW,
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub brand: Brand,
    pub price: f64,
    pub quantity: i32,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Wheel {
    pub brand: Brand,
    pub price: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Chassis {
    pub brand: Brand,
    pub price: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SoldCar {
    pub id: i32,
    pub brand: Brand,
    pub price: f64,
    pub created_at: i64,
}

pub trait Emittable: Serialize {
    fn generate() -> Self;
    fn get_topic_name() -> String;
    fn get_frequency() -> u64; 
}

impl Emittable for Order {
    fn generate() -> Order {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Order {
            id: rng.gen(),
            brand: match rng.gen_range(0..5) {
                0 => Brand::FERRARI,
                1 => Brand::RENAULT,
                2 => Brand::PEUGEOT,
                3 => Brand::CITROEN,
                _ => Brand::BMW,
            },
            price: rng.gen_range(10000.0..100000.0),
            quantity: rng.gen_range(1..10),
            created_at: chrono::Utc::now().timestamp(),
        }
    }

    fn get_topic_name() -> String {
        "Order".to_owned()
    }
    fn get_frequency() -> u64 {
        20
    }
}

impl Emittable for Wheel {
    fn generate() -> Wheel {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Wheel {
            brand: match rng.gen_range(0..5) {
                0 => Brand::FERRARI,
                1 => Brand::RENAULT,
                2 => Brand::PEUGEOT,
                3 => Brand::CITROEN,
                _ => Brand::BMW,
            },
            price: rng.gen_range(100.0..1000.0),
        }
    }
    fn get_topic_name() -> String {
        "Wheel".to_owned()
    }
    fn get_frequency() -> u64 {
        20
    }
}

impl Emittable for Chassis {
     fn generate() -> Chassis {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Chassis {
            brand: match rng.gen_range(0..5) {
                0 => Brand::FERRARI,
                1 => Brand::RENAULT,
                2 => Brand::PEUGEOT,
                3 => Brand::CITROEN,
                _ => Brand::BMW,
            },
            price: rng.gen_range(5000.0..20000.0),
        }
    }
    fn get_topic_name() -> String {
        "Chassis".to_owned()
    }
    fn get_frequency() -> u64 {
        20
    }
}
