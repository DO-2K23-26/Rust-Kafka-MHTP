use Rust_Kafka_MHT::common::consumer;
use datafusion::prelude::*;

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let mut consumer = consumer::get_consumer("first-course")?;
    loop {
    for ms in consumer.poll().unwrap().iter() {
      for m in ms.messages() {
        let str = String::from_utf8_lossy(m.value);
        println!("{:?}",str); // put message in datafusion table
      }
      let _ = consumer.consume_messageset(ms);
    }
    consumer.commit_consumed().unwrap();
  }
}