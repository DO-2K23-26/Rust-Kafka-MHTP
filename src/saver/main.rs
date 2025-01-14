use datafusion::arrow::array::{Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::physical_plan::memory::MemoryExec;
use datafusion::prelude::SessionContext;
use datafusion::prelude::*;
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

    loop {
        let mut messages = Vec::new();
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let message_text = String::from_utf8_lossy(m.value).to_string();
                println!("{:?}", message_text);
                messages.push(message_text);
            }
            let _ = consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
        if !messages.is_empty() {   
            let array = Arc::new(StringArray::from(messages)) as Arc<dyn Array>;
            let record_batch = RecordBatch::try_new(schema.clone(), vec![array])?;
            let execution_plan = Arc::new(MemoryExec::try_new(
                &[vec![record_batch]],
                schema.clone(),
                None,
            )?);

            ctx.write_parquet(execution_plan, "kafka_output.parquet", None)
                .await?;
            let dataframe = ctx.read_parquet("kafka_output.parquet", ParquetReadOptions::default()).await?;
            dataframe.show().await?;
        }
    }
}
