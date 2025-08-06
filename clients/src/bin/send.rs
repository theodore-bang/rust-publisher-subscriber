use pub_api::*;

fn main() {
    let pid = register_publisher().unwrap();

    let topic = "A Topic Name of 20B!";
    create_topic(pid, topic);

    let message = "I am a message of 30 bytes!!!!";

    loop {
        send(pid, topic, message);
    }
}