use datafusion::prelude::SessionContext;
// use aws_credential_types::provider::Credentials;
use Rust_Kafka_MHT::common::consumer;
use Rust_Kafka_MHT::common::types::SoldCar;

mod batch_converter;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut consumer = consumer::get_consumer("Order")?;
    let ctx = SessionContext::new();

    // const MINIO_ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
    // const MINIO_SECRET_ACCESS_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
    // const PROVIDER_NAME: &str = "Static";
    // const MINIO_ENDPOINT: &str = "http://localhost:9001";

    // let s3_file_system = S3FileSystem::new(
    //     Some(SharedCredentialsProvider::new(Credentials::new(
    //         MINIO_ACCESS_KEY_ID,
    //         MINIO_SECRET_ACCESS_KEY,
    //         None,
    //         None,
    //         PROVIDER_NAME,
    //     ))), // Credentials provider
    //     None,                                                        // Region
    //     Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))), // Endpoint
    //     None,                                                        // RetryConfig
    //     None,                                                        // AsyncSleep
    //     None,                                                        // TimeoutConfig
    // )
    // .await;
    // let s3_url = Url::parse("s3://")?;
    // let s3_store: Arc<dyn ObjectStore> = Arc::new(s3_file_system);
    // ctx.register_object_store(&s3_url, s3_store);

    loop {
        let mut sold_cars: Vec<SoldCar> = Vec::new();

        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let message_text = match serde_json::from_slice::<SoldCar>(m.value) {
                    Ok(message) => message,
                    Err(e) => {
                        eprintln!("Error while deserializing message: {:?}", e);
                        continue;
                    }
                };
                sold_cars.push(message_text);
            }
            let _ = consumer.consume_messageset(ms);
        }

        consumer.commit_consumed().unwrap();

        if !sold_cars.is_empty() {
            let execution_plan = batch_converter::convert(sold_cars)?;
            ctx.write_parquet(execution_plan, "s3://bucket/kafka_output.parquet", None)
                .await?;
            // let dataframe = ctx
            //     .read_parquet(
            //         "s3://bucket/kafka_output.parquet",
            //         ParquetReadOptions::default(),
            //     )
            //     .await?;
            // dataframe.show().await?;
        }
    }
}
