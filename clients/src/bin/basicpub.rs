use pub_api;

fn main() {
    let pid = pub_api::register_publisher().unwrap();

    let topic1 = "My First Topic";
    let topic2 = "My Second Topic";
    
    let message1 = "My first message!";
    let message2 = "My second message!";
    let message3 = "My third message!";

    pub_api::create_topic(pid, &topic1);
    pub_api::send(pid, &topic1, &message1);

    pub_api::create_topic(pid, &topic2);
    pub_api::send(pid, &topic2, &message1);
    pub_api::send(pid, &topic2, &message2);
    pub_api::send(pid, &topic2, &message3);
}