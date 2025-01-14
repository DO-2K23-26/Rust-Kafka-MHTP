# Rust-Kafka-MHT

Projet Rust Kafka Théo Mathias Hugo

## Data structure

Topic boss request:

```json
{
    "id": 1
    "model_name": "Clio5",
    "brand_name": "Renault",
    "needed_parts": [
        {
            "part_name": "engine",
            "quantity": 1
        },
        {
            "part_name": "wheel",
            "quantity": 4
        }
    ],
    "timestamp": 123456789
}
```

Topic parts arrivals:

```json
{
    "id": 1
    "part_name": "engine",
    "quantity": 1,
    "timestamp": 123456789
    "price" : 1000
}
```

Topic client orders:

```json
{
    "id": 1
    "model_name": "Clio5",
    "market_price": 15000,
    "timestamp": 123456789
}
```





## Utils

Read messages from a topic: 

```bash
docker exec --workdir /opt/kafka/bin/ -it kafka sh ./kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic Order --from-beginning
```

Send a message in a thread:
```bash
docker exec --workdir /opt/kafka/bin/ -it kafka sh ./kafka-console-producer.sh --bootstrap-server localhost:9092 --topic Order
```