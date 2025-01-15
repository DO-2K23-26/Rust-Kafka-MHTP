/// Handles the merge logic.
use std::time::SystemTime;

use crate::common::types::{Chassis, Brand, Order, Wheel, SoldCar, InBuildingCar};

pub fn check_brand(order: Order, chassis: Chassis) -> bool {
    order.brand == chassis.brand
}

pub fn calculate_price(order: Order, chassis: Chassis, wheels: [Wheel; 4], brand: Brand) -> f64 {
    let mut price = order.price + chassis.price;
    for wheel in wheels.iter() {
        price += wheel.price;
    }
    if brand == Brand::FERRARI {  // The price of luxury.
        price *= 1.2;
    }
    price
}

pub fn merge(order: Order, chassis: Chassis, brand: &Brand, wheels: [Wheel; 4]) -> SoldCar {
    SoldCar {
        id: order.id,
        brand: brand.clone(),  // Order brand and chassis brand must be the same before merging.
        price: calculate_price(order, chassis, wheels, brand.clone()),
        created_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
    }
}

// Todo : refactor these three functions into a generic attach_components function

pub fn attach_wheel(wheel: Wheel, in_building_cars: Arc<RwLock<Vec<InBuildingCar>>>) {
    for in_building_car in in_building_cars.iter() {
        // If there are less than 4 wheels, attach the wheel to the first empty slot.
        for i in 0..4 {
            if in_building_car.wheels[i].brand == wheel.brand {
                // write the wheel to the in_building_car
                break;
            }
        }
    }
}

