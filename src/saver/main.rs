use aws_types::credentials::SharedCredentialsProvider;
use aws_types::Credentials;
// use aws_credential_types::provider::Credentials;
use aws_sdk_s3::Endpoint;
use datafusion::arrow::array::{Array, Int32Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::physical_plan::memory::MemoryExec;
use datafusion::prelude::SessionContext;
use Rust_Kafka_MHT::common::types::SoldCar;
use std::sync::Arc;
use Rust_Kafka_MHT::common::consumer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut consumer = consumer::get_consumer("Order")?;
    let ctx = SessionContext::new();
    let schema = Arc::new(Schema::new(vec![Field::new(
        "message",
        DataType::Utf8,
        false,
    )]));

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
                let message_text = String::from_utf8_lossy(m.value).to_string();
                println!("{:?}", message_text);
                sold_cars.push(message_text);
            }
            let _ = consumer.consume_messageset(ms);
        }
        
        consumer.commit_consumed().unwrap();
        
        if !sold_cars.is_empty() {
            let array_id: Vec<String> = sold_cars.iter().map(|m| m.id.to_string()).collect();
            let df_array_id = Arc::new(StringArray::from(array_id)); 
            let record_batch = RecordBatch::try_new(schema.clone(), vec![df_array_id])?;
            
            let execution_plan = Arc::new(MemoryExec::try_new(
                &[vec![record_batch]],
                schema.clone(),
                None,
            )?);

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
