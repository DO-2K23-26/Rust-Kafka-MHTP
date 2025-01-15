use std::{error::Error, sync::Arc};

use datafusion::{
    arrow::{
        array::{RecordBatch, StringArray},
        datatypes::{DataType, Field, Schema},
    },
    physical_plan::memory::MemoryExec,
};
use Rust_Kafka_MHT::common::types::SoldCar;

pub fn convert(sold_cars: Vec<SoldCar>) -> Result<Arc<MemoryExec>, Box<dyn Error>> {
    let schema = Arc::new(Schema::new(vec![Field::new(
        "message",
        DataType::Utf8,
        false,
    )]));

    let array_id: Vec<String> = sold_cars.iter().map(|m| m.id.to_string()).collect();

    let df_array_id = Arc::new(StringArray::from(array_id));

    let record_batch = RecordBatch::try_new(schema.clone(), vec![df_array_id])?;

    let execution_plan = Arc::new(MemoryExec::try_new(
        &[vec![record_batch]],
        schema.clone(),
        None,
    )?);
    Ok(execution_plan)
}
