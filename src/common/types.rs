use serde::{Deserialize, Serialize};
use serde_avro_derive::BuildSchema;
use std::fmt::{Debug, Display, Formatter};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq, BuildSchema)]
pub enum Brand {
    FERRARI,
    RENAULT,
    PEUGEOT,
    CITROEN,
    #[default]
    BMW,
}

pub enum Component {
    ORDER(Order),
    CHASSIS(Chassis),
    WHEEL(Wheel),
}

#[derive(Serialize, Deserialize, Clone, Copy, BuildSchema)]
pub struct Order {
    pub id: i32,
    pub brand: Brand,
    pub price: f64,
    pub quantity: i32,
    pub created_at: u64,
}

impl Debug for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Order {{ id: {}, brand: {}, price: {}, quantity: {}, created_at: {} }}",
            self.id, self.brand, self.price, self.quantity, self.created_at
        )
    }
}

#[derive(Serialize, Deserialize, Clone, BuildSchema, Copy)]
pub struct Wheel {
    pub brand: Brand,
    pub price: f64,
}

impl Debug for Wheel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Wheel {{ brand: {}, price: {} }}",
            self.brand, self.price
        )
    }
}

#[derive(Serialize, Deserialize, Clone, BuildSchema, Copy)]
pub struct Chassis {
    pub brand: Brand,
    pub price: f64,
}

impl Debug for Chassis {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Chassis {{ brand: {}, price: {} }}",
            self.brand, self.price
        )
    }
}

pub trait Consumable: for<'de> Deserialize<'de> {
    fn get_topic_name() -> String;
    fn to_component(self) -> Component;
}

impl Consumable for Order {
    fn get_topic_name() -> String {
        "Order".to_owned()
    }

    fn to_component(self) -> Component {
        Component::ORDER(self)
    }
}

impl Consumable for Chassis {
    fn get_topic_name() -> String {
        "Chassis".to_owned()
    }

    fn to_component(self) -> Component {
        Component::CHASSIS(self)
    }
}

impl Consumable for Wheel {
    fn get_topic_name() -> String {
        "Wheel".to_owned()
    }

    fn to_component(self) -> Component {
        Component::WHEEL(self)
    }
}

#[derive(Serialize, Deserialize, Default, Copy, Clone, BuildSchema)]
pub struct SoldCar {
    pub id: i32,
    pub brand: Brand,
    pub price: f64,
    pub created_at: u64,
}

pub trait Emittable: Serialize {
    fn generate() -> Self;
    fn get_topic_name() -> String;
    fn get_frequency() -> u64;
}

impl Emittable for SoldCar {
    fn generate() -> SoldCar {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let brand = match rng.gen_range(0..100) {
            0..=4 => Brand::FERRARI, // 5% chance
            5..=29 => Brand::RENAULT, // 25% chance
            30..=54 => Brand::PEUGEOT, // 25% chance
            55..=79 => Brand::CITROEN, // 25% chance
            _ => Brand::BMW, // 20% chance
        };

        let price = match brand {
            Brand::FERRARI => rng.gen_range(80000.0..150000.0),
            Brand::RENAULT => rng.gen_range(20000.0..40000.0),
            Brand::PEUGEOT => rng.gen_range(25000.0..50000.0),
            Brand::CITROEN => rng.gen_range(22000.0..45000.0),
            Brand::BMW => rng.gen_range(30000.0..80000.0),
        };

        SoldCar {
            id: rng.gen(),
            brand,
            price,
            created_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    fn get_topic_name() -> String {
        "SoldCar".to_owned()
    }
    fn get_frequency() -> u64 {
        1
    }
}

impl Emittable for Order {
    fn generate() -> Order {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let brand = match rng.gen_range(0..100) {
            0..=4 => Brand::FERRARI, // 5% chance
            5..=29 => Brand::RENAULT, // 25% chance
            30..=54 => Brand::PEUGEOT, // 25% chance
            55..=79 => Brand::CITROEN, // 25% chance
            _ => Brand::BMW, // 20% chance
        };

        let (price, quantity) = match brand {
            Brand::FERRARI => (rng.gen_range(80000.0..150000.0), rng.gen_range(1..3)),
            Brand::RENAULT => (rng.gen_range(10000.0..30000.0), rng.gen_range(5..10)),
            Brand::PEUGEOT => (rng.gen_range(15000.0..35000.0), rng.gen_range(3..7)),
            Brand::CITROEN => (rng.gen_range(12000.0..32000.0), rng.gen_range(4..8)),
            Brand::BMW => (rng.gen_range(30000.0..80000.0), rng.gen_range(2..5)),
        };

        Order {
            id: rng.gen(),
            brand,
            price,
            quantity,
            created_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
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

        let brand = match rng.gen_range(0..100) {
            0..=4 => Brand::FERRARI, // 5% chance
            5..=29 => Brand::RENAULT, // 25% chance
            30..=54 => Brand::PEUGEOT, // 25% chance
            55..=79 => Brand::CITROEN, // 25% chance
            _ => Brand::BMW, // 20% chance
        };

        let price = match brand {
            Brand::FERRARI => rng.gen_range(800.0..1000.0),
            Brand::RENAULT => rng.gen_range(100.0..300.0),
            Brand::PEUGEOT => rng.gen_range(200.0..400.0),
            Brand::CITROEN => rng.gen_range(150.0..350.0),
            Brand::BMW => rng.gen_range(300.0..600.0),
        };

        Wheel {
            brand,
            price,
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

        let brand = match rng.gen_range(0..100) {
            0..=4 => Brand::FERRARI, // 5% chance
            5..=29 => Brand::RENAULT, // 25% chance
            30..=54 => Brand::PEUGEOT, // 25% chance
            55..=79 => Brand::CITROEN, // 25% chance
            _ => Brand::BMW, // 20% chance
        };

        let price = match brand {
            Brand::FERRARI => rng.gen_range(15000.0..30000.0),
            Brand::RENAULT => rng.gen_range(5000.0..10000.0),
            Brand::PEUGEOT => rng.gen_range(7000.0..12000.0),
            Brand::CITROEN => rng.gen_range(6000.0..11000.0),
            Brand::BMW => rng.gen_range(10000.0..20000.0),
        };

        Chassis {
            brand,
            price,
        }
    }
    fn get_topic_name() -> String {
        "Chassis".to_owned()
    }
    fn get_frequency() -> u64 {
        20
    }
}

impl Display for Brand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Brand::FERRARI => write!(f, "FERRARI"),
            Brand::RENAULT => write!(f, "RENAULT"),
            Brand::PEUGEOT => write!(f, "PEUGEOT"),
            Brand::CITROEN => write!(f, "CITROEN"),
            Brand::BMW => write!(f, "BMW"),
        }
    }
}
