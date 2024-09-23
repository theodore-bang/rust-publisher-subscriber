use common;
use pub_api;

fn main() {
    let pid = pub_api::register_publisher().unwrap();
    let topic = "FooBar".to_string();
    let message = "Well, hello there!".to_string();

    pub_api::create_topic(pid, topic.clone());
    pub_api::send(pid, topic.clone(), message.clone());
}
