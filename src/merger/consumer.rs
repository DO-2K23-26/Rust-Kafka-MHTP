use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use schema_registry_converter::async_impl::avro::AvroDecoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinSet;

use crate::common::types::{Consumable, SoldCar};
use crate::merger::logic::InBuildingCar;

use super::logic::{check_mergeable, Buildable};

pub async fn create_consumer<T>(
    client_set: Arc<RwLock<JoinSet<()>>>,
    in_building_cars: Arc<RwLock<Vec<InBuildingCar>>>,
    sold_cars: Arc<RwLock<Vec<SoldCar>>>,
) where
    T: Consumable + Send + Sync + Clone + Copy + 'static,
{
    let mut consumer = Consumer::from_hosts(vec!["localhost:19092".to_owned(), "localhost:29092".to_owned()])
        .with_topic_partitions(T::get_topic_name().to_owned(), &[0])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("merger".to_owned())
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .unwrap();

    let mut client_set_lock = client_set.write().await;
    client_set_lock.spawn(async move {
        let sr_settings = SrSettings::new("http://localhost:8085".to_string());
        let decoder = AvroDecoder::new(sr_settings);
        loop {
            for ms in consumer.poll().unwrap().iter() {
                // Retrieve the MessageSet. There is at most one per topic.
                for m in ms.messages() {
                    
                    let decoded_value =  decoder.decode(Some(m.value)).await.unwrap();
                    let decoded: T = match serde_json::from_value(Value::try_from(decoded_value.value).unwrap()){
                        Ok(value) => value,
                        Err(e) => {
                            eprintln!("Error while deserializing message: {:?}", e);
                            continue;
                        }
                    };
                    let mut in_building_cars = in_building_cars.write().await;

                    // Check if in_building_cars is empty. If it's empty, push an empty InBuildingCar to be built.
                    if in_building_cars.is_empty() {
                        in_building_cars.push(InBuildingCar::default());
                    }

                    let mut attached = false;
                    for car in in_building_cars.iter_mut() {
                        if car.attach_component(decoded.to_component()) {
                            attached = true;
                            break;
                        }
                    }

                    if !attached {
                        let mut new_car = InBuildingCar::default();
                        let new_attached = new_car.attach_component(decoded.to_component());
                        log::debug!("Attached component to new car: {}", new_attached);

                        if new_attached {
                            in_building_cars.push(new_car);
                        }
                    }

                    let mut sold_cars = sold_cars.write().await;
                    check_mergeable(&mut *in_building_cars, &mut *sold_cars);
                    log::debug!("length of in_building_cars: {}", in_building_cars.len());

                    // Commit the single consumed message only if the component was attached.
                    consumer.consume_message(&T::get_topic_name(), 0, m.offset).unwrap();
                    consumer.commit_consumed().unwrap();
                }
            }
        }
    });
}
