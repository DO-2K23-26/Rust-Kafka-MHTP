# Merger service

## What is the Merger?

The Merger service is a kafka producer and consumer client.  
It consumes from three topics:

- Chassis
- Wheel
- Order

And produces to one topic:

- SoldCar

## Why do we need the Merger?

The Merger output will eventually be retrieved and analyzed by the dashboards service.  
It will first be retrieved and stored by the saver service.

In itself, the Merger builds and sells the cars, using kafka as a buffer and losing no components or orders.

## How is the Merger implemented?


Implementation notes:

inBuildingCars are cars that are being built. They may be stored in a list in the memory of the program.  
When a car is done, it is removed from the list and sent back to the soldCars topic.

## Possible issues and solutions

- Problem N°1: What if you're stuck waiting for a topic element?
  - -> You consumed an order, but the corresponding elements (chassis, wheels) never arrive. You are stuck waiting.

- Possible solution for this problem:
  - You have to keep looping on reading the topic to see if the missing element becomes available
  - Or you use tokio to run each merging operation asynchronously in different threads

---

- Problem N°2: What if your container dies while you're building cars?
  - -> The topic elements have already been consumed. The inBuildingCars list in memory is lost.  
  - This means we lose an order and its components.

- Possible solution for this problem: "event sourcing".
  - Create a new inBuildingCars topic. Instead of using a list, the Merger service transfers each element and order to this topic.
  - Kafka handles buffering the elements, the Merger only transfers them.
