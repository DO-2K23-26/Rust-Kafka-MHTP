use std::{fmt::Write, time::Duration};

use kafka::{
    client::RequiredAcks,
    producer::{Producer, Record},
};

fn main() {
    let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let mut buf = String::with_capacity(2);
    loop {
        let _ = write!(&mut buf, "{}", "test");
        producer
            .send(&Record::from_value("first-course", buf.as_bytes()))
            .unwrap();
        buf.clear();
        std::thread::sleep(Duration::from_secs(1));
        println!("test")
    }
}
