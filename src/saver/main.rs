use std::sync::Arc;

use datafusion::execution::object_store::ObjectStoreUrl;
use datafusion::prelude::SessionContext;
use object_store::aws::AmazonS3Builder;
// use aws_credential_types::provider::Credentials;
use Rust_Kafka_MHT::common::consumer;
use Rust_Kafka_MHT::common::types::SoldCar;

mod batch_converter;

const MAX_BATCH_SIZE: usize = 4000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut consumer = consumer::get_consumer("SoldCar")?;
    let ctx = SessionContext::new();

    const MINIO_ACCESS_KEY_ID: &str = "6UKbinEQGNB7hh2ftxMo";
    const MINIO_SECRET_ACCESS_KEY: &str = "MHASPodqbmFH9tgeGWE4hHiJYkBnWsCTQuxJvLEq";

    let s3 = AmazonS3Builder::new()
        .with_region("us-east-1")
        .with_bucket_name("datafusion")
        .with_access_key_id(MINIO_ACCESS_KEY_ID)
        .with_secret_access_key(MINIO_SECRET_ACCESS_KEY)
        .with_endpoint("http://localhost:9000/")
        .with_allow_http(true)
        .build()?;
    let object_store_url = ObjectStoreUrl::parse("s3://datafusion/").unwrap();
    ctx.register_object_store(&object_store_url.as_ref(), Arc::new(s3));
    let mut sold_cars: [SoldCar; MAX_BATCH_SIZE] = [SoldCar::default(); MAX_BATCH_SIZE];
    let mut cursor = 0;

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let sold_car = match serde_json::from_slice::<SoldCar>(m.value) {
                    Ok(message) => message,
                    Err(e) => {
                        eprintln!("Error while deserializing message: {:?}", e);
                        continue;
                    }
                };
                sold_cars[cursor] = sold_car;
                cursor += 1;
                if cursor == MAX_BATCH_SIZE {
                    let execution_plan = batch_converter::convert(sold_cars.to_vec())?;
                    ctx.write_parquet(execution_plan, "s3://datafusion/output/", None).await?;
                    println!("wrote parquet");
                    cursor = 0;
                };
            }
            let _ = consumer.consume_messageset(ms);
            consumer.commit_consumed().unwrap();
        }
    }
}
