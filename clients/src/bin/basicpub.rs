use pub_api::*;

fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}
fn main() {

    let first_topic = "FIRST TOPIC";
    let second_topic = "SECOND TOPIC";
    
    let pub1 = register_publisher().unwrap();
    let pub2 = register_publisher().unwrap();

    create_topic(pub1, first_topic);
    create_topic(pub2, second_topic);

    sleep (1);

    send(pub1, first_topic, "first message");
    send(pub1, first_topic, "second message");
    send(pub1, first_topic, "third message");
    send(pub2, second_topic, "first message");
    send(pub2, second_topic, "second message");

    sleep(1);

    send(pub1, first_topic, "more message");
    send(pub1, first_topic, "even more messages");

}