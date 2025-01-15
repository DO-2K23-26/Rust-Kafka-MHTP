use std::{error::Error, sync::Arc};

use datafusion::{
    arrow::{
        array::{Float64Array, Int64Array, RecordBatch, StringArray},
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
    let array_brand: Vec<String> = sold_cars.iter().map(|m| m.brand.to_string()).collect();
    let array_price: Vec<f64> = sold_cars.iter().map(|m| m.price).collect();
    let array_created_at: Vec<i64> = sold_cars.iter().map(|m| m.created_at).collect();

    let df_array_id = Arc::new(StringArray::from(array_id));
    let df_array_brand = Arc::new(StringArray::from(array_brand));
    let df_array_price = Arc::new(Float64Array::from(array_price));
    let df_array_created_at = Arc::new(Int64Array::from(array_created_at));

    let record_batch = RecordBatch::try_new(schema.clone(), vec![df_array_id, df_array_brand, df_array_price, df_array_created_at])?;
    let execution_plan = Arc::new(MemoryExec::try_new(
        &[vec![record_batch]],
        schema.clone(),
        None,
    )?);
    Ok(execution_plan)
}
