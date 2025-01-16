use datafusion::dataframe::DataFrameWriteOptions;
use schema_registry_converter::async_impl::avro::AvroDecoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use serde_json::Value;
use std::sync::Arc;

use datafusion::execution::object_store::ObjectStoreUrl;
use datafusion::prelude::SessionContext;
use kafka::error::KafkaCode;
use object_store::aws::AmazonS3Builder;
// use aws_credential_types::provider::Credentials;
use Rust_Kafka_MHT::common::consumer;
use Rust_Kafka_MHT::common::types::SoldCar;

mod batch_converter;
const MAX_BATCH_SIZE: usize = 4000;
const NUMBER_OF_PARTITION: i32 = 2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = vec![];

    for partition_id in 0..NUMBER_OF_PARTITION{
        println!("Launching consumer for partition {}", partition_id);
        let handle = tokio::spawn(async move {
            let mut consumer = match consumer::get_consumer("SoldCar", partition_id) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error getting consumer: {:?}", e);
                    return Err(Box::new(e));
                }
            };
            let ctx = SessionContext::new();

            const MINIO_ACCESS_KEY_ID: &str = "minio_access_key";
            const MINIO_SECRET_ACCESS_KEY: &str = "minio_secret_key";

            let s3 = match AmazonS3Builder::new()
                .with_region("us-east-1")
                .with_bucket_name("data-bucket")
                .with_access_key_id(MINIO_ACCESS_KEY_ID)
                .with_secret_access_key(MINIO_SECRET_ACCESS_KEY)
                .with_endpoint("http://localhost:9000/")
                .with_allow_http(true)
                .build() {
                Ok(s3) => s3,
                Err(_) => {
                    return Err(Box::new(kafka::Error::Kafka(KafkaCode::ClusterAuthorizationFailed)));
                }
            };
            let object_store_url = ObjectStoreUrl::parse("s3://data-bucket").unwrap();
            ctx.register_object_store(&object_store_url.as_ref(), Arc::new(s3));

            let mut sold_cars: [SoldCar; MAX_BATCH_SIZE] = [SoldCar::default(); MAX_BATCH_SIZE];
            let mut cursor = 0;
            
            let sr_settings = SrSettings::new("http://localhost:8085".to_string());
            let decoder = AvroDecoder::new(sr_settings);
            loop {
                for ms in consumer.poll().unwrap().iter() {
                    for m in ms.messages() {
                        let decoded_value =  decoder.decode(Some(m.value)).await.unwrap();
                        let sold_car: SoldCar = match serde_json::from_value(Value::try_from(decoded_value.value).unwrap()){
                            Ok(value) => value,
                            Err(e) => {
                                eprintln!("Error while deserializing message: {:?}", e);
                                continue;
                            }
                        };
                        sold_cars[cursor] = sold_car;
                        cursor += 1;
                        if cursor == MAX_BATCH_SIZE {
                            let execution_plan = match batch_converter::convert(sold_cars.to_vec()) {
                                Ok(plan) => plan,
                                Err(e) => {
                                    eprintln!("Error converting batch: {:?}", e);
                                    return Err(Box::new(kafka::Error::Kafka(KafkaCode::ClusterAuthorizationFailed)));
                                }
                            };
                            let df = ctx.read_batch(execution_plan).unwrap();
                            let file_name = format!("s3://data-bucket/data/{}{}{}.parquet", "SoldCar", partition_id, m.offset);
                            match df.write_parquet(&file_name, DataFrameWriteOptions::new(), None).await {
                                Ok(_) => println!("wrote parquet"),
                                Err(e) => {
                                    eprintln!("Error writing parquet: {:?}", e);
                                    return Err(Box::new(kafka::Error::Kafka(KafkaCode::ClusterAuthorizationFailed)));
                                }
                            };
                            cursor = 0;
                        };
                    }
                    let _ = consumer.consume_messageset(ms);
                    consumer.commit_consumed().unwrap();
                }
            }
        });

        handles.push(handle);
    }

    println!("Waiting for all handles to finish");

    for handle in handles {
        handle.await??;
    }

    Ok(())
}