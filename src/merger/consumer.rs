use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
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
    T: Consumable + Send + Sync + Clone +'static,
{
    let consumer = Consumer::from_hosts(vec!["localhost:19092".to_owned(),"localhost:29092".to_owned()])
        .with_topic_partitions(T::get_topic_name().to_owned(), &[0])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("merger".to_owned())
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .unwrap();
    let consumer_arc = Arc::new(RwLock::new(consumer));
    let consumer_clone = consumer_arc.clone();

    let mut client_set_lock = client_set.write().await;
    client_set_lock.spawn(async move {
        let mut consumer_lock = consumer_clone.write().await;
        loop {
            for ms in consumer_lock.poll().unwrap().iter() {
                // Retrieve the MessageSet. There is at most one per topic.
                for m in ms.messages() {
                    let decoded = serde_json::from_slice::<T>(m.value).unwrap();
                    let mut in_building_cars = in_building_cars.write().await;

                    // Attach the component to the first car if the list isn't empty.
                    if let Some(car) = in_building_cars.first_mut() {
                        car.attach_component(decoded.to_component());
                    }

                    let mut sold_cars = sold_cars.write().await;
                    check_mergeable(&mut *in_building_cars, &mut *sold_cars);
                }
                let _ =  consumer_lock.consume_messageset(ms);
            }
            consumer_lock.commit_consumed().unwrap();
        }
    });
}
