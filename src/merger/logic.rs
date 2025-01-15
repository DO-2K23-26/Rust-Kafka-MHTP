/// Handles the merge logic.
use serde::{Serialize, Deserialize};

//use std::sync::Arc;
//use tokio::sync::RwLock;

use std::time::SystemTime;

use crate::common::types::{Brand, Chassis, Order, Wheel, SoldCar};
use crate::common::types::Component;

pub fn calculate_price(order: Order, chassis: Chassis, wheels: [Wheel; 4], brand: Brand) -> f64 {
    let mut price = order.price + chassis.price;
    for wheel in wheels.iter() {
        price += wheel.price;
    }
    if brand == Brand::FERRARI {
        price *= 1.2;  // The price of luxury.
    }
    price
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InBuildingCar {
    pub order: Option<Order>,
    pub chassis: Option<Chassis>,
    pub wheels: [Option<Wheel>; 4],
}

pub trait Buildable {
    fn attach_component(self: &mut Self, component: Component) -> bool;
    fn merge(order: Order, chassis: Chassis, brand: Brand, wheels: [Wheel; 4]) -> SoldCar;
}

impl Buildable for InBuildingCar {
    fn attach_component(self: &mut Self, component: Component) -> bool {
        match component {
            Component::ORDER(order) => {
                if self.order.is_none() {
                    self.order = Some(order);
                    return true;
                }
            }
            Component::CHASSIS(chassis) => {
                if self.chassis.is_none() {
                    self.chassis = Some(chassis);
                    return true;
                }
            }
            Component::WHEEL(wheel) => {
                for i in 0..4 {
                    if self.wheels[i].is_none() {
                        self.wheels[i] = Some(wheel);
                        return true;
                    }
                }
            }
        }
        false
    }

    fn merge(order: Order, chassis: Chassis, brand: Brand, wheels: [Wheel; 4]) -> SoldCar {
        SoldCar {
            id: order.id,
            brand: brand.clone(),  // Order brand, chassis brand and wheel brands must be the same before merging.
            price: calculate_price(order, chassis, wheels, brand.clone()),
            created_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

pub fn check_mergeable(in_building_cars: &mut Vec<InBuildingCar>, sold_cars: &mut Vec<SoldCar>) {
    let mut i = 0;
    while i < in_building_cars.len() {
        let in_building_car = in_building_cars.remove(i);
        if let (Some(ref order), Some(ref chassis), Some(ref wheel1), Some(ref wheel2), Some(ref wheel3), Some(ref wheel4)) =
            (&in_building_car.order, &in_building_car.chassis, &in_building_car.wheels[0], &in_building_car.wheels[1], &in_building_car.wheels[2], &in_building_car.wheels[3])
        {
            let brand = order.brand.clone();
            let wheels = [wheel1.clone(), wheel2.clone(), wheel3.clone(), wheel4.clone()];
            let sold_car = InBuildingCar::merge(order.clone(), chassis.clone(), brand, wheels);
            sold_cars.push(sold_car);
        } else {
            in_building_cars.insert(i, in_building_car);
            i += 1;
        }
    }
}
