pub enum Brand {
    FERRARI,
    RENAULT,
    PEUGEOT,
    CITROEN,
    BMW,
}

pub struct Order {
    pub brand: Brand,
    pub price: f64,
    pub quantity: i32,
    pub created_at: i64,
}

pub struct Wheel {
    pub brand: Brand,
    pub price: f64,
}

pub struct Chassis {
    pub brand: Brand,
    pub price: f64,
}
