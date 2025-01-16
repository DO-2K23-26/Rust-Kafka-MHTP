# Rust-Kafka-MHT

## How to run

### 1. Start the stack

```bash
docker-compose -f docker-compose.yml -f trino/docker-compose.yml up
```

### 2. Create the table in trino

```sql
CREATE TABLE hive.default.sold_car (
    id varchar, 
    brand VARCHAR, 
    price double, 
    created_at varchar
    )
    WITH (
        external_location = 's3a://data-bucket/data/',
        format = 'parquet'
    );
```

### 3. Build and run the Rust application

```bash
cargo build --bin producer --bin merger --bin saver
```

```bash
/target/debug/producer
/target/debug/merger
/target/debug/saver
```

### 4. Now you can access all the services and notably the grafana dashboard at http://localhost:3000
