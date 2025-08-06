use sub_api::*;

fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}
fn main() {
    let first_topic = "FIRST TOPIC";
    let second_topic = "SECOND TOPIC";

    let sub1 = register_subscriber().unwrap();

    sleep(1);

    subscribe(sub1, first_topic);
    subscribe(sub1, second_topic);

    sleep(1);
    
    let sub1_msgs1 = pull(sub1, first_topic);
    let sub1_msgs2 = pull(sub1, second_topic);

    for msg in sub1_msgs1 {
        println!("{sub1} received: {msg} from {first_topic}");
    }
    for msg in sub1_msgs2 {
        println!("{sub1} received: {msg} from {second_topic}");
    }

    println!("{sub1} waiting...");
    sleep(1);

    let sub1_msgs1 = pull(sub1, first_topic);

    for msg in sub1_msgs1 {
        println!("{sub1} received: {msg} from {first_topic}");
    }
}