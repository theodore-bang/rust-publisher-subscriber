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

    pub_api::create_topic(pid, &topic1);
    pub_api::send(pid, &topic1, &message1);

    pub_api::create_topic(pid, &topic2);
    pub_api::send(pid, &topic2, &message1);
    pub_api::send(pid, &topic2, &message2);
    pub_api::send(pid, &topic2, &message3);
}