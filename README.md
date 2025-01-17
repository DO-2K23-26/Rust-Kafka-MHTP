# Rust-Kafka-MHT

## How to run

### 1. Start the stack

```bash
docker-compose -f docker-compose.yml -f trino/docker-compose.yml up
```

### 2. Build and run the Rust application

```bash
cargo build --bin producer --bin merger --bin saver
```

```bash
/target/debug/producer
/target/debug/merger
/target/debug/saver
```

### 3. Now you can access all the services and notably the grafana dashboard at http://localhost:3000

### 4. You can save any of your dashboards (in json) on the folder `trino/config/grafana/dashboards` and they will be automatically loaded on the next restart