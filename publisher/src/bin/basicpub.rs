use pub_api;

fn main() {
    let pid = pub_api::register_publisher().unwrap();

    let topic1 = "My First Topic".to_string();
    let topic2 = "My Second Topic".to_string();
    let topic3 = "My Third Topic".to_string();
    
    let message1 = "My first message!".to_string();
    let message2 = "My second message!".to_string();
    let message3 = "My third message!".to_string();
    let message4 = "My fourth message!".to_string();

    pub_api::create_topic(pid, topic1.clone());
    pub_api::send(pid, topic1.clone(), message1.clone());

    pub_api::create_topic(pid, topic2.clone());
    pub_api::send(pid, topic2.clone(), message1.clone());
    pub_api::send(pid, topic2.clone(), message2.clone());
    pub_api::send(pid, topic2.clone(), message3.clone());
}