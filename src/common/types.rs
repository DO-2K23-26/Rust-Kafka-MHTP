pub enum Brand {
    FERRARI,
    RENAULT,
    PEUGEOT,
    CITROEN,
    BMW,
}

pub struct Order {
    pub id: Option<i32>,
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

pub struct SoldCar {
    pub id: Option<i32>,
    pub order_id: i32,
    pub brand: Brand,
    pub price: f64,
    pub created_at: i64,
}