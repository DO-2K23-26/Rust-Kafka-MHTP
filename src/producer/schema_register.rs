use schema_registry_converter::{
    async_impl::schema_registry::{post_schema, SrSettings},
    schema_registry_common::{SchemaType, SuppliedSchema},
};
use serde_avro_derive::BuildSchema;
use Rust_Kafka_MHT::common::types::Emittable;

pub async fn register<T>()
where
    T: Emittable + BuildSchema,
{
    let sr_settings = SrSettings::new(String::from("http://localhost:8085"));
    let schema = T::schema().unwrap();
    let supplied_schema = SuppliedSchema {
        name: Some(T::get_topic_name()),
        schema_type: SchemaType::Avro,
        schema: String::from(schema.json()),
        references: vec![],
    };

    if let Err(e) = post_schema(
        &sr_settings,
        format!("{}-value", T::get_topic_name()).to_string(),
        supplied_schema,
    )
    .await
    {
        eprintln!("Failed to post client schema: {}", e);
    };
}
